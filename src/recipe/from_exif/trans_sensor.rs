use crate::exif::{ExifData, FindExifData, FromExifData};
use crate::recipe::TransSensor;
use log::trace;

impl FromExifData for TransSensor {
    fn from_exif(data: &[ExifData]) -> Option<Self> {
        let exif = data.find("Model")?;

        trace!("TransSensor::from_exif: {:?}", exif);

        match exif.value().to_lowercase().as_str() {
            "x-pro1" => Some(TransSensor::TransI),
            "x-e1" => Some(TransSensor::TransI),
            "x-m1" => Some(TransSensor::TransI),
            "x100s" => Some(TransSensor::TransII),
            "x-e2" => Some(TransSensor::TransII),
            "x-t1" => Some(TransSensor::TransII),
            "x100t" => Some(TransSensor::TransII),
            "x-t10" => Some(TransSensor::TransII),
            "x-e2s" => Some(TransSensor::TransII),
            "x70" => Some(TransSensor::TransII),
            "x20" => Some(TransSensor::TransII),
            "xq1" => Some(TransSensor::TransII),
            "x30" => Some(TransSensor::TransII),
            "xq2" => Some(TransSensor::TransII),
            "x-pro2" => Some(TransSensor::TransIII),
            "x-t2" => Some(TransSensor::TransIII),
            "x100f" => Some(TransSensor::TransIII),
            "x-t20" => Some(TransSensor::TransIII),
            "x-e3" => Some(TransSensor::TransIII),
            "x-h1" => Some(TransSensor::TransIII),
            "x-t3" => Some(TransSensor::TransIV),
            "x-t4" => Some(TransSensor::TransIV),
            "x-t30" => Some(TransSensor::TransIV),
            "x-t30 ii" => Some(TransSensor::TransIV),
            "x-pro3" => Some(TransSensor::TransIV),
            "x100v" => Some(TransSensor::TransIV),
            "x-e4" => Some(TransSensor::TransIV),
            "x-s10" => Some(TransSensor::TransIV),
            "x-s20" => Some(TransSensor::TransIV),
            "x-m5" => Some(TransSensor::TransIV),
            "x-h2s" => Some(TransSensor::TransV),
            "x-h2" => Some(TransSensor::TransV),
            "x-t5" => Some(TransSensor::TransV),
            "x-t50" => Some(TransSensor::TransV),
            "x100vi" => Some(TransSensor::TransV),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_parses_trans_i_sensor() {
        let exif: Vec<ExifData> = vec![ExifData::new("Model", "X-Pro1")];
        assert_eq!(TransSensor::from_exif(&exif), Some(TransSensor::TransI));

        let exif: Vec<ExifData> = vec![ExifData::new("Model", "X-E1")];
        assert_eq!(TransSensor::from_exif(&exif), Some(TransSensor::TransI));

        let exif: Vec<ExifData> = vec![ExifData::new("Model", "X-M1")];
        assert_eq!(TransSensor::from_exif(&exif), Some(TransSensor::TransI));
    }

    #[test]
    fn it_parses_trans_ii_sensor() {
        let exif: Vec<ExifData> = vec![ExifData::new("Model", "X100S")];
        assert_eq!(TransSensor::from_exif(&exif), Some(TransSensor::TransII));

        let exif: Vec<ExifData> = vec![ExifData::new("Model", "X-E2")];
        assert_eq!(TransSensor::from_exif(&exif), Some(TransSensor::TransII));

        let exif: Vec<ExifData> = vec![ExifData::new("Model", "X-T1")];
        assert_eq!(TransSensor::from_exif(&exif), Some(TransSensor::TransII));

        let exif: Vec<ExifData> = vec![ExifData::new("Model", "X100T")];
        assert_eq!(TransSensor::from_exif(&exif), Some(TransSensor::TransII));

        let exif: Vec<ExifData> = vec![ExifData::new("Model", "X-T10")];
        assert_eq!(TransSensor::from_exif(&exif), Some(TransSensor::TransII));

        let exif: Vec<ExifData> = vec![ExifData::new("Model", "X-E2S")];
        assert_eq!(TransSensor::from_exif(&exif), Some(TransSensor::TransII));

        let exif: Vec<ExifData> = vec![ExifData::new("Model", "X70")];
        assert_eq!(TransSensor::from_exif(&exif), Some(TransSensor::TransII));

        let exif: Vec<ExifData> = vec![ExifData::new("Model", "X20")];
        assert_eq!(TransSensor::from_exif(&exif), Some(TransSensor::TransII));

        let exif: Vec<ExifData> = vec![ExifData::new("Model", "XQ1")];
        assert_eq!(TransSensor::from_exif(&exif), Some(TransSensor::TransII));

        let exif: Vec<ExifData> = vec![ExifData::new("Model", "X30")];
        assert_eq!(TransSensor::from_exif(&exif), Some(TransSensor::TransII));

        let exif: Vec<ExifData> = vec![ExifData::new("Model", "XQ2")];
        assert_eq!(TransSensor::from_exif(&exif), Some(TransSensor::TransII));
    }

    #[test]
    fn it_parses_trans_iii_sensor() {
        let exif: Vec<ExifData> = vec![ExifData::new("Model", "X-Pro2")];
        assert_eq!(TransSensor::from_exif(&exif), Some(TransSensor::TransIII));

        let exif: Vec<ExifData> = vec![ExifData::new("Model", "X-T2")];
        assert_eq!(TransSensor::from_exif(&exif), Some(TransSensor::TransIII));

        let exif: Vec<ExifData> = vec![ExifData::new("Model", "X100F")];
        assert_eq!(TransSensor::from_exif(&exif), Some(TransSensor::TransIII));

        let exif: Vec<ExifData> = vec![ExifData::new("Model", "X-T20")];
        assert_eq!(TransSensor::from_exif(&exif), Some(TransSensor::TransIII));

        let exif: Vec<ExifData> = vec![ExifData::new("Model", "X-E3")];
        assert_eq!(TransSensor::from_exif(&exif), Some(TransSensor::TransIII));

        let exif: Vec<ExifData> = vec![ExifData::new("Model", "X-H1")];
        assert_eq!(TransSensor::from_exif(&exif), Some(TransSensor::TransIII));
    }

    #[test]
    fn it_parses_trans_iv_sensor() {
        let exif: Vec<ExifData> = vec![ExifData::new("Model", "X-T3")];
        assert_eq!(TransSensor::from_exif(&exif), Some(TransSensor::TransIV));

        let exif: Vec<ExifData> = vec![ExifData::new("Model", "X-T4")];
        assert_eq!(TransSensor::from_exif(&exif), Some(TransSensor::TransIV));

        let exif: Vec<ExifData> = vec![ExifData::new("Model", "X-T30")];
        assert_eq!(TransSensor::from_exif(&exif), Some(TransSensor::TransIV));

        let exif: Vec<ExifData> = vec![ExifData::new("Model", "X-T30 II")];
        assert_eq!(TransSensor::from_exif(&exif), Some(TransSensor::TransIV));

        let exif: Vec<ExifData> = vec![ExifData::new("Model", "X-Pro3")];
        assert_eq!(TransSensor::from_exif(&exif), Some(TransSensor::TransIV));

        let exif: Vec<ExifData> = vec![ExifData::new("Model", "X100V")];
        assert_eq!(TransSensor::from_exif(&exif), Some(TransSensor::TransIV));

        let exif: Vec<ExifData> = vec![ExifData::new("Model", "X-S10")];
        assert_eq!(TransSensor::from_exif(&exif), Some(TransSensor::TransIV));

        let exif: Vec<ExifData> = vec![ExifData::new("Model", "X-S20")];
        assert_eq!(TransSensor::from_exif(&exif), Some(TransSensor::TransIV));

        let exif: Vec<ExifData> = vec![ExifData::new("Model", "X-E4")];
        assert_eq!(TransSensor::from_exif(&exif), Some(TransSensor::TransIV));

        let exif: Vec<ExifData> = vec![ExifData::new("Model", "X-M5")];
        assert_eq!(TransSensor::from_exif(&exif), Some(TransSensor::TransIV));
    }

    #[test]
    fn it_parses_trans_v_sensor() {
        let exif: Vec<ExifData> = vec![ExifData::new("Model", "X-H2S")];
        assert_eq!(TransSensor::from_exif(&exif), Some(TransSensor::TransV));

        let exif: Vec<ExifData> = vec![ExifData::new("Model", "X-H2")];
        assert_eq!(TransSensor::from_exif(&exif), Some(TransSensor::TransV));

        let exif: Vec<ExifData> = vec![ExifData::new("Model", "X-T5")];
        assert_eq!(TransSensor::from_exif(&exif), Some(TransSensor::TransV));

        let exif: Vec<ExifData> = vec![ExifData::new("Model", "X-T50")];
        assert_eq!(TransSensor::from_exif(&exif), Some(TransSensor::TransV));

        let exif: Vec<ExifData> = vec![ExifData::new("Model", "X100VI")];
        assert_eq!(TransSensor::from_exif(&exif), Some(TransSensor::TransV));
    }

    #[test]
    fn it_does_not_parse_when_not_found() {
        let exif: Vec<ExifData> = vec![ExifData::new("Foo", "X100VI")];
        assert_eq!(TransSensor::from_exif(&exif), None);
    }
}
