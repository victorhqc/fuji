extern crate test_utilities;

use fuji::{
    exiftool::spawn,
    recipe::{
        read, Clarity, Color, ColorChromeEffect, ColorChromeEffectFxBlue, DRangePriority,
        DynamicRange, FilmSimulation, FujifilmRecipeDetails, GrainEffect, GrainSize, GrainStrength,
        HighISONoiseReduction, MonochromaticColor, SettingStrength, Settings, Sharpness, ToneCurve,
        TransSensor, TransVSettings, WBShift, WhiteBalance,
    },
};
use pretty_assertions::assert_eq;
use test_utilities::get_manifest_dir;

#[test]
fn test_classic_neg() {
    let img_path = get_manifest_dir().join("tests/img/DSCF5230.JPG");

    let result = spawn::read_metadata(img_path.as_path(), None).unwrap();
    let recipe = read::from_exif(result.as_slice()).unwrap();

    let expected = Some(FujifilmRecipeDetails {
        film_simulation: FilmSimulation::ClassicNeg,
        sensor: TransSensor::TransV,
        settings: Settings::TransV(TransVSettings {
            white_balance: WhiteBalance::Kelvin {
                temperature: 4700,
                shift: WBShift { red: 4, blue: -2 },
            },
            dynamic_range: DynamicRange::DR200,
            d_range_priority: DRangePriority::default(),
            grain_effect: GrainEffect::StrengthAndSize {
                strength: GrainStrength::Strong,
                size: GrainSize::Small,
            },
            color_chrome_effect: ColorChromeEffect {
                strength: SettingStrength::Strong,
            },
            color_chrome_fx_blue: ColorChromeEffectFxBlue::default(),
            tone_curve: ToneCurve {
                highlights: -2.,
                shadows: 0.,
            },
            high_iso_noise_reduction: HighISONoiseReduction { value: -4 },
            color: Color { value: 4 },
            sharpness: Sharpness::default(),
            clarity: Clarity::default(),
            monochromatic_color: MonochromaticColor::default(),
        }),
    });
    assert_eq!(recipe, expected);
}

#[test]
fn test_classic_chrome() {
    let img_path = get_manifest_dir().join("tests/img/DSCF5358.JPG");

    let result = spawn::read_metadata(img_path.as_path(), None).unwrap();
    let recipe = read::from_exif(result.as_slice()).unwrap();

    let expected = Some(FujifilmRecipeDetails {
        film_simulation: FilmSimulation::ClassicChrome,
        sensor: TransSensor::TransV,
        settings: Settings::TransV(TransVSettings {
            white_balance: WhiteBalance::Kelvin {
                temperature: 5600,
                shift: WBShift { red: 1, blue: 1 },
            },
            dynamic_range: DynamicRange::DR200,
            d_range_priority: DRangePriority::default(),
            grain_effect: GrainEffect::StrengthAndSize {
                strength: GrainStrength::Weak,
                size: GrainSize::Large,
            },
            color_chrome_effect: ColorChromeEffect {
                strength: SettingStrength::Strong,
            },
            color_chrome_fx_blue: ColorChromeEffectFxBlue::default(),
            tone_curve: ToneCurve {
                highlights: -2.,
                shadows: 1.,
            },
            high_iso_noise_reduction: HighISONoiseReduction { value: -4 },
            color: Color { value: 3 },
            sharpness: Sharpness::default(),
            clarity: Clarity { value: 2 },
            monochromatic_color: MonochromaticColor::default(),
        }),
    });
    assert_eq!(recipe, expected);
}
