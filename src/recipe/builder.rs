use crate::exif::{ExifData, FromExifData};
use crate::recipe::{
    Clarity, Color, ColorChromeEffect, ColorChromeEffectFxBlue, DRangePriority, DynamicRange,
    GrainEffect, HighISONoiseReduction, MonochromaticColor, Settings, Sharpness, ToneCurve,
    TransIIISettings, TransIISettings, TransISettings, TransIVSettings, TransSensor,
    TransVSettings, WhiteBalance,
};

#[derive(Default)]
pub struct SettingsBuilder {
    white_balance: Option<WhiteBalance>,
    dynamic_range: Option<DynamicRange>,
    d_range_priority: Option<DRangePriority>,
    grain_effect: Option<GrainEffect>,
    color_chrome_effect: Option<ColorChromeEffect>,
    color_chrome_fx_blue: Option<ColorChromeEffectFxBlue>,
    tone_curve: Option<ToneCurve>,
    color: Option<Color>,
    monochromatic_color: Option<MonochromaticColor>,
    sharpness: Option<Sharpness>,
    high_iso_noise_reduction: Option<HighISONoiseReduction>,
    clarity: Option<Clarity>,
}

impl SettingsBuilder {
    pub fn build_for_i(self) -> TransISettings {
        TransISettings {
            white_balance: self.white_balance.unwrap_or_default(),
            dynamic_range: self.dynamic_range.unwrap_or_default(),
            tone_curve: self.tone_curve.unwrap_or_default(),
            color: self.color.unwrap_or_default(),
            sharpness: self.sharpness.unwrap_or_default(),
            high_iso_noise_reduction: self.high_iso_noise_reduction.unwrap_or_default(),
        }
    }

    pub fn build_for_ii(self) -> TransIISettings {
        TransIISettings {
            dynamic_range: self.dynamic_range.unwrap_or_default(),
            white_balance: self.white_balance.unwrap_or_default(),
            tone_curve: self.tone_curve.unwrap_or_default(),
            color: self.color.unwrap_or_default(),
            sharpness: self.sharpness.unwrap_or_default(),
            high_iso_noise_reduction: self.high_iso_noise_reduction.unwrap_or_default(),
        }
    }

    pub fn build_for_iii(self) -> TransIIISettings {
        TransIIISettings {
            white_balance: self.white_balance.unwrap_or_default(),
            dynamic_range: self.dynamic_range.unwrap_or_default(),
            grain_effect: self.grain_effect.unwrap_or_default(),
            tone_curve: self.tone_curve.unwrap_or_default(),
            color: self.color.unwrap_or_default(),
            monochromatic_color: self.monochromatic_color.unwrap_or_default(),
            sharpness: self.sharpness.unwrap_or_default(),
            high_iso_noise_reduction: self.high_iso_noise_reduction.unwrap_or_default(),
        }
    }

    pub fn build_for_iv(self) -> TransIVSettings {
        TransIVSettings {
            white_balance: self.white_balance.unwrap_or_default(),
            dynamic_range: self.dynamic_range.unwrap_or_default(),
            d_range_priority: self.d_range_priority.unwrap_or_default(),
            grain_effect: self.grain_effect.unwrap_or_default(),
            color_chrome_fx_blue: self.color_chrome_fx_blue.unwrap_or_default(),
            color_chrome_effect: self.color_chrome_effect.unwrap_or_default(),
            tone_curve: self.tone_curve.unwrap_or_default(),
            color: self.color.unwrap_or_default(),
            monochromatic_color: self.monochromatic_color.unwrap_or_default(),
            sharpness: self.sharpness.unwrap_or_default(),
            high_iso_noise_reduction: self.high_iso_noise_reduction.unwrap_or_default(),
            clarity: self.clarity.unwrap_or_default(),
        }
    }

    pub fn build_for_v(self) -> TransVSettings {
        TransVSettings {
            white_balance: self.white_balance.unwrap_or_default(),
            dynamic_range: self.dynamic_range.unwrap_or_default(),
            d_range_priority: self.d_range_priority.unwrap_or_default(),
            grain_effect: self.grain_effect.unwrap_or_default(),
            color_chrome_effect: self.color_chrome_effect.unwrap_or_default(),
            color_chrome_fx_blue: self.color_chrome_fx_blue.unwrap_or_default(),
            tone_curve: self.tone_curve.unwrap_or_default(),
            color: self.color.unwrap_or_default(),
            monochromatic_color: self.monochromatic_color.unwrap_or_default(),
            sharpness: self.sharpness.unwrap_or_default(),
            high_iso_noise_reduction: self.high_iso_noise_reduction.unwrap_or_default(),
            clarity: self.clarity.unwrap_or_default(),
        }
    }

    pub fn with_white_balance(&mut self, white_balance: Option<WhiteBalance>) -> &mut Self {
        self.white_balance = white_balance;
        self
    }

    pub fn with_dynamic_range(&mut self, dynamic_range: Option<DynamicRange>) -> &mut Self {
        self.dynamic_range = dynamic_range;
        self
    }

    pub fn with_d_range_priority(&mut self, d_range_priority: Option<DRangePriority>) -> &mut Self {
        self.d_range_priority = d_range_priority;
        self
    }

    pub fn with_grain_effect(&mut self, grain_effect: Option<GrainEffect>) -> &mut Self {
        self.grain_effect = grain_effect;
        self
    }

    pub fn with_color_chrome_effect(
        &mut self,
        color_chrome_effect: Option<ColorChromeEffect>,
    ) -> &mut Self {
        self.color_chrome_effect = color_chrome_effect;
        self
    }

    pub fn with_color_chrome_fx_blue(
        &mut self,
        color_chrome_fx_blue: Option<ColorChromeEffectFxBlue>,
    ) -> &mut Self {
        self.color_chrome_fx_blue = color_chrome_fx_blue;
        self
    }

    pub fn with_tone_curve(&mut self, tone_curve: Option<ToneCurve>) -> &mut Self {
        self.tone_curve = tone_curve;
        self
    }

    pub fn with_color(&mut self, color: Option<Color>) -> &mut Self {
        self.color = color;
        self
    }

    pub fn with_monochromatic_color(
        &mut self,
        monochromatic_color: Option<MonochromaticColor>,
    ) -> &mut Self {
        self.monochromatic_color = monochromatic_color;
        self
    }

    pub fn with_sharpness(&mut self, sharpness: Option<Sharpness>) -> &mut Self {
        self.sharpness = sharpness;
        self
    }

    pub fn with_high_iso_noise_reduction(
        &mut self,
        high_iso_noise_reduction: Option<HighISONoiseReduction>,
    ) -> &mut Self {
        self.high_iso_noise_reduction = high_iso_noise_reduction;
        self
    }

    pub fn with_clarity(&mut self, clarity: Option<Clarity>) -> &mut Self {
        self.clarity = clarity;
        self
    }
}

impl FromExifData for SettingsBuilder {
    fn from_exif(data: &[ExifData]) -> Option<Self> {
        let white_balance = WhiteBalance::from_exif(data);
        let dynamic_range = DynamicRange::from_exif(data);
        let d_range_priority = DRangePriority::from_exif(data);
        let grain_effect = GrainEffect::from_exif(data);
        let color_chrome_effect = ColorChromeEffect::from_exif(data);
        let color_chrome_fx_blue = ColorChromeEffectFxBlue::from_exif(data);
        let tone_curve = ToneCurve::from_exif(data);
        let color = Color::from_exif(data);
        let monochromatic_color = MonochromaticColor::from_exif(data);
        let sharpness = Sharpness::from_exif(data);
        let high_iso_noise_reduction = HighISONoiseReduction::from_exif(data);
        let clarity = Clarity::from_exif(data);

        let builder = SettingsBuilder {
            white_balance,
            dynamic_range,
            d_range_priority,
            grain_effect,
            color_chrome_effect,
            color_chrome_fx_blue,
            tone_curve,
            color,
            monochromatic_color,
            sharpness,
            high_iso_noise_reduction,
            clarity,
        };

        Some(builder)
    }
}

impl TransSensor {
    pub fn settings(&self, builder: SettingsBuilder) -> Settings {
        match self {
            TransSensor::TransI => Settings::TransI(builder.build_for_i()),
            TransSensor::TransII => Settings::TransII(builder.build_for_ii()),
            TransSensor::TransIII => Settings::TransIII(builder.build_for_iii()),
            TransSensor::TransIV => Settings::TransIV(builder.build_for_iv()),
            TransSensor::TransV => Settings::TransV(builder.build_for_v()),
        }
    }
}
