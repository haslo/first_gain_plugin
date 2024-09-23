# First Gain Plugin

A simple audio gain plugin built with NIH-plug. Dual panning and gain per channel, logarithmic scaling for the gain sliders.

Very rudimentary UI.

## Building

To build the plugin (VST3, CLAP), install Rust and then use use:

```
cargo xtask bundle dual_panning_plugin --release
```

This should pull the NIH-plug repo. After the build, the VST3 and CLAP build will be in target/bundled, ready to be moved to your plugin folders.

## Development

This project uses the NIH-plug framework. Refer to the [NIH-plug documentation](https://github.com/robbert-vdh/nih-plug) for more information on development and customization.

## License

This project uses GPL because the VST3 bindings used in this plugin, via the `nih_export_vst3!()` macro, are licensed under GPLv3.

For more information, refer to the licensing terms in the [NIH-plug documentation](https://github.com/robbert-vdh/nih-plug?tab=readme-ov-file#licensing).
