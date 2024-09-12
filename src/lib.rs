use nih_plug::prelude::*;
use nih_plug_egui::EguiState;
use std::sync::Arc;

struct FirstGainPlugin {
    params: Arc<FirstGainParams>,
}

#[derive(Params)]
struct FirstGainParams {
    #[id = "gain"]
    pub gain: FloatParam,

    #[persist = "editor-state"]
    editor_state: Arc<EguiState>,
}

impl Default for FirstGainPlugin {
    fn default() -> Self {
        Self {
            params: Arc::new(FirstGainParams::default()),
        }
    }
}

impl Default for FirstGainParams {
    fn default() -> Self {
        Self {
            gain: FloatParam::new(
                "Gain",
                0.0,
                FloatRange::Linear {
                    min: -12.0,
                    max: 12.0,
                },
            )
                .with_unit(" dB")
                .with_value_to_string(formatters::v2s_f32_gain_to_db(2))
                .with_string_to_value(formatters::s2v_f32_gain_to_db()),
            editor_state: EguiState::from_size(200, 60),
        }
    }
}

impl Plugin for FirstGainPlugin {
    const NAME: &'static str = "haslo's First Gain Plugin";
    const VENDOR: &'static str = "haslo";
    const URL: &'static str = "https://haslo.ch/";
    const EMAIL: &'static str = "";

    const VERSION: &'static str = env!("CARGO_PKG_VERSION");

    const AUDIO_IO_LAYOUTS: &'static [AudioIOLayout] = &[
        AudioIOLayout {
            main_input_channels: NonZeroU32::new(2),
            main_output_channels: NonZeroU32::new(2),
            ..AudioIOLayout::const_default()
        },
    ];

    const MIDI_INPUT: MidiConfig = MidiConfig::None;
    const MIDI_OUTPUT: MidiConfig = MidiConfig::None;

    const SAMPLE_ACCURATE_AUTOMATION: bool = true;

    type SysExMessage = ();
    type BackgroundTask = ();

    fn params(&self) -> Arc<dyn Params> {
        self.params.clone()
    }

    fn editor(&mut self, _async_executor: AsyncExecutor<Self>) -> Option<Box<dyn Editor>> {
        use nih_plug_egui::egui;
        nih_plug_egui::create_egui_editor(
            self.params.editor_state.clone(),
            self.params.clone(),
            |_, _| (), // Default state initialization callback
            move |egui_ctx, setter, state| {
                egui::CentralPanel::default().show(egui_ctx, |ui| {
                    ui.heading("haslo's First Gain Plugin");
                    let mut gain = state.gain.value();
                    if ui.add(egui::Slider::new(&mut gain, -12.0..=12.0).text("Gain")).changed() {
                        setter.begin_set_parameter(&state.gain);
                        setter.set_parameter(&state.gain, gain);
                        setter.end_set_parameter(&state.gain);
                    }
                });
            },
        )
    }

    fn process(
        &mut self,
        buffer: &mut Buffer,
        _aux: &mut AuxiliaryBuffers,
        _context: &mut impl ProcessContext<Self>,
    ) -> ProcessStatus {
        for channel_samples in buffer.iter_samples() {
            let gain = self.params.gain.smoothed.next();
            let gain_linear = util::db_to_gain(gain);
            for sample in channel_samples {
                *sample *= gain_linear;
            }
        }
        ProcessStatus::Normal
    }
}

impl ClapPlugin for FirstGainPlugin {
    const CLAP_ID: &'static str = "ch.haslo.first-gain-plugin";
    const CLAP_DESCRIPTION: Option<&'static str> = Some("A simple gain plugin");
    const CLAP_MANUAL_URL: Option<&'static str> = Some("https://haslo.ch/");
    const CLAP_SUPPORT_URL: Option<&'static str> = None;
    const CLAP_FEATURES: &'static [ClapFeature] = &[ClapFeature::AudioEffect, ClapFeature::Stereo];
}

impl Vst3Plugin for FirstGainPlugin {
    const VST3_CLASS_ID: [u8; 16] = [0x12, 0x34, 0x56, 0x78, 0x9A, 0xBC, 0xDE, 0xF0, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88];
    const VST3_SUBCATEGORIES: &'static [Vst3SubCategory] = &[Vst3SubCategory::Fx];
}

nih_export_clap!(FirstGainPlugin);
nih_export_vst3!(FirstGainPlugin);
