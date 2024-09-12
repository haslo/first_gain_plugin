# First Gain Plugin

A simple audio gain plugin built with NIH-plug.

## Building

To build the plugin (VST3, CLAP), use:

```
cargo xtask bundle first_gain_plugin --release
```

## Development

This project uses the NIH-plug framework. Refer to the [NIH-plug documentation](https://github.com/robbert-vdh/nih-plug) for more information on development and customization.

## License

The source code for this project is licensed under the MIT License. However, the VST3 bindings used in this plugin, via the `nih_export_vst3!()` macro, are licensed under GPLv3. As a result, any distributed binary of this plugin must comply with the terms of the GPLv3 license.

For more information, refer to the licensing terms in the [NIH-plug documentation](https://github.com/robbert-vdh/nih-plug?tab=readme-ov-file#licensing).
