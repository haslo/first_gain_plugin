use nih_plug::prelude::*;
use nih_plug_egui::EguiState;
use std::sync::Arc;

struct DualPanningPlugin {
    params: Arc<DualPanningParams>,
}

#[derive(Params)]
struct DualPanningParams {
    #[id = "left_volume"]
    pub left_volume: FloatParam,

    #[id = "right_volume"]
    pub right_volume: FloatParam,

    #[id = "left_pan"]
    pub left_pan: FloatParam,

    #[id = "right_pan"]
    pub right_pan: FloatParam,

    #[persist = "editor-state"]
    editor_state: Arc<EguiState>,
}

impl Default for DualPanningPlugin {
    fn default() -> Self {
        Self {
            params: Arc::new(DualPanningParams::default()),
        }
    }
}

impl Default for DualPanningParams {
    fn default() -> Self {
        Self {
            left_volume: FloatParam::new(
                "Left Volume",
                0.0,
                FloatRange::Linear {
                    min: -12.0,
                    max: 12.0,
                },
            )
                .with_unit(" dB")
                .with_value_to_string(formatters::v2s_f32_gain_to_db(2))
                .with_string_to_value(formatters::s2v_f32_gain_to_db()),
            right_volume: FloatParam::new(
                "Right Volume",
                0.0,
                FloatRange::Linear {
                    min: -12.0,
                    max: 12.0,
                },
            )
                .with_unit(" dB")
                .with_value_to_string(formatters::v2s_f32_gain_to_db(2))
                .with_string_to_value(formatters::s2v_f32_gain_to_db()),
            left_pan: FloatParam::new(
                "Left Pan",
                -1.0,
                FloatRange::Linear {
                    min: -1.0,
                    max: 1.0,
                },
            ),
            right_pan: FloatParam::new(
                "Right Pan",
                1.0,
                FloatRange::Linear {
                    min: -1.0,
                    max: 1.0,
                },
            ),
            editor_state: EguiState::from_size(400, 300),
        }
    }
}

impl Plugin for DualPanningPlugin {
    const NAME: &'static str = "Dual Panning Plugin";
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
                    ui.heading("Dual Panning Plugin");
                    ui.add_space(10.0);

                    ui.horizontal(|ui| {
                        // Left half
                        ui.vertical(|ui| {
                            ui.label("Left Pan");
                            let mut left_pan = state.left_pan.value();
                            if ui.add(egui::Slider::new(&mut left_pan, -1.0..=1.0).text("L")).changed() {
                                setter.begin_set_parameter(&state.left_pan);
                                setter.set_parameter(&state.left_pan, left_pan);
                                setter.end_set_parameter(&state.left_pan);
                            }
                            ui.add_space(10.0);
                            ui.label("Left Gain");
                            let mut left_volume = state.left_volume.value();
                            if ui.add(egui::Slider::new(&mut left_volume, -12.0..=12.0).vertical().text("L")).changed() {
                                setter.begin_set_parameter(&state.left_volume);
                                setter.set_parameter(&state.left_volume, left_volume);
                                setter.end_set_parameter(&state.left_volume);
                            }
                        });

                        ui.separator();

                        // Right half
                        ui.vertical(|ui| {
                            ui.label("Right Pan");
                            let mut right_pan = state.right_pan.value();
                            if ui.add(egui::Slider::new(&mut right_pan, -1.0..=1.0).text("R")).changed() {
                                setter.begin_set_parameter(&state.right_pan);
                                setter.set_parameter(&state.right_pan, right_pan);
                                setter.end_set_parameter(&state.right_pan);
                            }
                            ui.add_space(10.0);
                            ui.label("Right Gain");
                            let mut right_volume = state.right_volume.value();
                            if ui.add(egui::Slider::new(&mut right_volume, -12.0..=12.0).vertical().text("R")).changed() {
                                setter.begin_set_parameter(&state.right_volume);
                                setter.set_parameter(&state.right_volume, right_volume);
                                setter.end_set_parameter(&state.right_volume);
                            }
                        });
                    });
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
        for mut channel_samples in buffer.iter_samples() {
            let left_gain = util::db_to_gain(self.params.left_volume.smoothed.next());
            let right_gain = util::db_to_gain(self.params.right_volume.smoothed.next());
            let left_pan = self.params.left_pan.smoothed.next();
            let right_pan = self.params.right_pan.smoothed.next();

            // Assume stereo - iterate over pairs of samples
            let mut iter = channel_samples.iter_mut();
            if let (Some(left), Some(right)) = (iter.next(), iter.next()) {
                let left_sample = *left;
                let right_sample = *right;

                // Apply volume and panning
                *left = (left_sample * left_gain * (1.0 - left_pan).max(0.0)) +
                    (right_sample * right_gain * (1.0 - right_pan).max(0.0));
                *right = (left_sample * left_gain * (1.0 + left_pan).max(0.0)) +
                    (right_sample * right_gain * (1.0 + right_pan).max(0.0));
            }
        }
        ProcessStatus::Normal
    }
}

impl ClapPlugin for DualPanningPlugin {
    const CLAP_ID: &'static str = "ch.haslo.dual-panning-plugin";
    const CLAP_DESCRIPTION: Option<&'static str> = Some("A dual panning plugin");
    const CLAP_MANUAL_URL: Option<&'static str> = Some("https://haslo.ch/");
    const CLAP_SUPPORT_URL: Option<&'static str> = None;
    const CLAP_FEATURES: &'static [ClapFeature] = &[ClapFeature::AudioEffect, ClapFeature::Stereo];
}

impl Vst3Plugin for DualPanningPlugin {
    const VST3_CLASS_ID: [u8; 16] = [0x12, 0x34, 0x56, 0x78, 0x9A, 0xBC, 0xDE, 0xF0, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88];
    const VST3_SUBCATEGORIES: &'static [Vst3SubCategory] = &[Vst3SubCategory::Fx];
}

nih_export_clap!(DualPanningPlugin);
nih_export_vst3!(DualPanningPlugin);
