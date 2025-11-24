use serde::{Deserialize, Serialize};

use crate::{
    elements::BackboneElement,
    resources::{self, ResourceType},
};

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all(deserialize = "kebab-case", serialize = "lowercase"))]
pub enum DaysOfWeek {
    Mon,
    Tue,
    Wed,
    Thu,
    Fri,
    Sat,
    Sun,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all(serialize = "snake_case", deserialize = "camelCase"))]
pub struct AvailableTime {
    #[serde(flatten)]
    pub backbone_element: BackboneElement,
    pub days_of_week: Option<Vec<DaysOfWeek>>,
    pub all_day: Option<bool>,
    pub available_start_time: Option<String>, // to be addressed later
    pub available_end_time: Option<String>,   // to be addressed later
}

impl ResourceType for AvailableTime {
    const TYPE: &'static str = "AvailableTime";
}

impl AvailableTime {
    pub fn from_json(data: &str) -> Self {
        resources::from_json(data)
    }
}

#[derive(Default)]
pub struct AvailableTimeBuilder {
    backbone_element: BackboneElement,
    days_of_week: Option<Vec<DaysOfWeek>>,
    all_day: Option<bool>,
    available_start_time: Option<String>, // to be addressed later
    available_end_time: Option<String>,   // to be addressed later
}

impl AvailableTimeBuilder {
    pub fn new(id: impl Into<String>) -> Self {
        let mut builder = Self::default();
        builder.backbone_element.element.id = Some(id.into());
        builder
    }

    pub fn with_days_of_week(mut self, days_of_week: Vec<DaysOfWeek>) -> Self {
        self.days_of_week = Some(days_of_week);
        self
    }

    pub fn add_day_of_week(mut self, day: DaysOfWeek) -> Self {
        match &mut self.days_of_week {
            Some(dow) => dow.push(day),
            None => self.days_of_week = Some(vec![day]),
        }
        self
    }

    pub fn with_all_day(mut self, all_day: bool) -> Self {
        self.all_day = Some(all_day);
        self
    }

    pub fn with_available_start_time(mut self, start_time: impl Into<String>) -> Self {
        self.available_start_time = Some(start_time.into());
        self
    }

    pub fn with_available_end_time(mut self, end_time: impl Into<String>) -> Self {
        self.available_end_time = Some(end_time.into());
        self
    }

    pub fn build(self) -> AvailableTime {
        AvailableTime {
            backbone_element: self.backbone_element,
            days_of_week: self.days_of_week,
            all_day: self.all_day,
            available_start_time: self.available_start_time,
            available_end_time: self.available_end_time,
        }
    }
}

#[cfg(test)]
mod test {
    

    use crate::elements::Element;

    use super::*;

    #[test]
    fn test_from_json_should_succeed() {
        let data = r#"{
                "daysOfWeek": ["mon", "tue", "wed", "thu", "fri"],
                "availableStartTime": "08:00",
                "availableEndTime": "17:00"
            }"#;

        let expected = AvailableTimeBuilder::default()
            .with_days_of_week(vec![
                DaysOfWeek::Mon,
                DaysOfWeek::Tue,
                DaysOfWeek::Wed,
                DaysOfWeek::Thu,
                DaysOfWeek::Fri,
            ])
            .with_available_start_time("08:00")
            .with_available_end_time("17:00")
            .build();
        let actual = AvailableTime::from_json(data);

        assert_eq!(expected, actual)
    }

    #[test]
    fn test_build_should_succeed() {
        let expected = AvailableTime {
            backbone_element: BackboneElement {
                element: Element {
                    id: Some("av-1".to_string()),
                    ..Default::default()
                },
                ..Default::default()
            },
            days_of_week: Some(vec![DaysOfWeek::Mon, DaysOfWeek::Tue]),
            available_start_time: Some("08:00".to_string()),
            available_end_time: Some("12:00".to_string()),
            all_day: Some(true),
        };

        let actual = AvailableTimeBuilder::new("av-1")
            .with_all_day(true)
            .with_available_start_time("08:00")
            .with_available_end_time("12:00")
            .add_day_of_week(DaysOfWeek::Mon)
            .add_day_of_week(DaysOfWeek::Tue)
            .build();

        assert_eq!(expected, actual)
    }
}
