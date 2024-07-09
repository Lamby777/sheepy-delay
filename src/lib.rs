use nih_plug::prelude::*;
use std::sync::Arc;

mod ringbuffer;
use ringbuffer::RingBuffer;

struct SheepyDelay {
    params: Arc<SheepyDelayParams>,
    ring_buffer: RingBuffer<f32>,
}

#[derive(Params)]
struct SheepyDelayParams {
    #[id = "wet"]
    pub wet: FloatParam,
}

impl Default for SheepyDelay {
    fn default() -> Self {
        Self {
            params: Arc::new(SheepyDelayParams::default()),
            ring_buffer: RingBuffer::new(44100),
        }
    }
}

impl Default for SheepyDelayParams {
    fn default() -> Self {
        Self {
            wet: FloatParam::new("Wet", 0.0, FloatRange::Linear { min: 0.0, max: 1.0 })
                .with_unit("%")
                // There are many predefined formatters we can use here. If the gain was stored as
                // decibels instead of as a linear gain value, we could have also used the
                // `.with_step_size(0.1)` function to get internal rounding.
                .with_value_to_string(formatters::v2s_f32_percentage(2))
                .with_string_to_value(formatters::s2v_f32_percentage()),
        }
    }
}

impl Plugin for SheepyDelay {
    const NAME: &'static str = "Sheepy Delay";
    const VENDOR: &'static str = "Cherry";
    const URL: &'static str = env!("CARGO_PKG_HOMEPAGE");
    const EMAIL: &'static str = "refcherry@sparklet.org";

    const VERSION: &'static str = env!("CARGO_PKG_VERSION");

    // The first audio IO layout is used as the default. The other layouts may be selected either
    // explicitly or automatically by the host or the user depending on the plugin API/backend.
    const AUDIO_IO_LAYOUTS: &'static [AudioIOLayout] = &[AudioIOLayout {
        main_input_channels: NonZeroU32::new(2),
        main_output_channels: NonZeroU32::new(2),

        aux_input_ports: &[],
        aux_output_ports: &[],

        // Individual ports and the layout as a whole can be named here. By default these names
        // are generated as needed. This layout will be called 'Stereo', while a layout with
        // only one input and output channel would be called 'Mono'.
        names: PortNames::const_default(),
    }];

    const MIDI_INPUT: MidiConfig = MidiConfig::None;
    const MIDI_OUTPUT: MidiConfig = MidiConfig::None;

    const SAMPLE_ACCURATE_AUTOMATION: bool = true;

    // If the plugin can send or receive SysEx messages, it can define a type to wrap around those
    // messages here. The type implements the `SysExMessage` trait, which allows conversion to and
    // from plain byte buffers.
    type SysExMessage = ();
    // More advanced plugins can use this to run expensive background tasks. See the field's
    // documentation for more information. `()` means that the plugin does not have any background
    // tasks.
    type BackgroundTask = ();

    fn params(&self) -> Arc<dyn Params> {
        self.params.clone()
    }

    fn initialize(
        &mut self,
        _audio_io_layout: &AudioIOLayout,
        _buffer_config: &BufferConfig,
        _context: &mut impl InitContext<Self>,
    ) -> bool {
        // Resize buffers and perform other potentially expensive initialization operations here.
        // The `reset()` function is always called right after this function. You can remove this
        // function if you do not need it.
        true
    }

    fn reset(&mut self) {
        // Reset buffers and envelopes here. This can be called from the audio thread and may not
        // allocate. You can remove this function if you do not need it.
        self.ring_buffer.clear();
    }

    fn process(
        &mut self,
        buffer: &mut Buffer,
        _aux: &mut AuxiliaryBuffers,
        _context: &mut impl ProcessContext<Self>,
    ) -> ProcessStatus {
        for channel_samples in buffer.iter_samples() {
            // Smoothing is optionally built into the parameters themselves
            let wet = self.params.wet.smoothed.next();

            for sample in channel_samples {
                *sample *= wet;
            }
        }

        ProcessStatus::Normal
    }
}

impl ClapPlugin for SheepyDelay {
    const CLAP_ID: &'static str = "org.sparklet.sheepy-delay";
    const CLAP_DESCRIPTION: Option<&'static str> = Some("Delay plugin for learning purposes");
    const CLAP_MANUAL_URL: Option<&'static str> = Some(Self::URL);
    const CLAP_SUPPORT_URL: Option<&'static str> = None;

    // Don't forget to change these features
    const CLAP_FEATURES: &'static [ClapFeature] = &[ClapFeature::Delay];
}

impl Vst3Plugin for SheepyDelay {
    const VST3_CLASS_ID: [u8; 16] = *b"CherrySheepDelay";

    // And also don't forget to change these categories
    const VST3_SUBCATEGORIES: &'static [Vst3SubCategory] =
        &[Vst3SubCategory::Fx, Vst3SubCategory::Delay];
}

nih_export_clap!(SheepyDelay);
nih_export_vst3!(SheepyDelay);
