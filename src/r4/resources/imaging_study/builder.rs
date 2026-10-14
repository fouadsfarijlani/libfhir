use crate::r4::{
    elements::{BackboneElement, CodeableConcept, Identifier, Reference},
    resources::{
        DomainResource, Endpoint, ImagingStudy, ImagingStudyInstance,
        ImagingStudySeries, ImagingStudyStatus, Patient, Practitioner, Resource, ResourceType,
    },
};

#[derive(Default)]
pub struct ImagingStudyInstanceBuilder {
    backbone_element: BackboneElement,
    uid: String,
    number: Option<u32>,
    sop_class: Option<String>,
    title: Option<String>,
}

impl ImagingStudyInstanceBuilder {
    pub fn new(id: impl Into<String>) -> Self {
        let mut builder = Self::default();
        builder.backbone_element.element.id = Some(id.into());
        builder
    }

    pub fn id(mut self, id: impl Into<String>) -> Self {
        self.backbone_element.element.id = Some(id.into());
        self
    }

    pub fn uid(mut self, uid: impl Into<String>) -> Self {
        self.uid = uid.into();
        self
    }

    pub fn number(mut self, number: u32) -> Self {
        self.number = Some(number);
        self
    }

    pub fn sop_class(mut self, sop_class: impl Into<String>) -> Self {
        self.sop_class = Some(sop_class.into());
        self
    }

    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }

    pub fn build(self) -> ImagingStudyInstance {
        ImagingStudyInstance {
            backbone_element: self.backbone_element,
            uid: self.uid,
            number: self.number,
            sop_class: self.sop_class,
            title: self.title,
        }
    }
}

#[derive(Default)]
pub struct ImagingStudySeriesBuilder {
    backbone_element: BackboneElement,
    uid: String,
    number: Option<u32>,
    modality: CodeableConcept,
    description: Option<String>,
    number_of_instances: Option<u32>,
    endpoint: Option<Vec<Reference<Endpoint>>>,
    body_site: Option<CodeableConcept>,
    laterality: Option<CodeableConcept>,
    started: Option<String>,
    instance: Option<Vec<ImagingStudyInstance>>,
}

impl ImagingStudySeriesBuilder {
    pub fn new(id: impl Into<String>) -> Self {
        let mut builder = Self::default();
        builder.backbone_element.element.id = Some(id.into());
        builder
    }

    pub fn id(mut self, id: impl Into<String>) -> Self {
        self.backbone_element.element.id = Some(id.into());
        self
    }

    pub fn uid(mut self, uid: impl Into<String>) -> Self {
        self.uid = uid.into();
        self
    }

    pub fn number(mut self, number: u32) -> Self {
        self.number = Some(number);
        self
    }

    pub fn modality(mut self, modality: CodeableConcept) -> Self {
        self.modality = modality;
        self
    }

    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    pub fn number_of_instances(mut self, number: u32) -> Self {
        self.number_of_instances = Some(number);
        self
    }

    pub fn endpoint(mut self, endpoint: Vec<Reference<Endpoint>>) -> Self {
        self.endpoint = Some(endpoint);
        self
    }

    pub fn add_endpoint(mut self, endpoint: Reference<Endpoint>) -> Self {
        match &mut self.endpoint {
            Some(eps) => eps.push(endpoint),
            None => self.endpoint = Some(vec![endpoint]),
        }
        self
    }

    pub fn body_site(mut self, body_site: CodeableConcept) -> Self {
        self.body_site = Some(body_site);
        self
    }

    pub fn laterality(mut self, laterality: CodeableConcept) -> Self {
        self.laterality = Some(laterality);
        self
    }

    pub fn started(mut self, started: impl Into<String>) -> Self {
        self.started = Some(started.into());
        self
    }

    pub fn instance(mut self, instance: Vec<ImagingStudyInstance>) -> Self {
        self.instance = Some(instance);
        self
    }

    pub fn add_instance(mut self, instance: ImagingStudyInstance) -> Self {
        match &mut self.instance {
            Some(inst) => inst.push(instance),
            None => self.instance = Some(vec![instance]),
        }
        self
    }

    pub fn build(self) -> ImagingStudySeries {
        ImagingStudySeries {
            backbone_element: self.backbone_element,
            uid: self.uid,
            number: self.number,
            modality: self.modality,
            description: self.description,
            number_of_instances: self.number_of_instances,
            endpoint: self.endpoint,
            body_site: self.body_site,
            laterality: self.laterality,
            started: self.started,
            instance: self.instance,
        }
    }
}

#[derive(Default)]
pub struct ImagingStudyBuilder {
    domain_resource: DomainResource,
    identifier: Option<Vec<Identifier>>,
    status: Option<ImagingStudyStatus>,
    modality: Option<Vec<CodeableConcept>>,
    r#type: Option<CodeableConcept>,
    subject: Option<Reference<Patient>>,
    started: Option<String>,
    based_on: Option<Vec<Reference<Practitioner>>>,
    referrer: Option<Reference<Practitioner>>,
    interpreter: Option<Vec<Reference<Practitioner>>>,
    endpoint: Option<Vec<Reference<Endpoint>>>,
    number_of_series: Option<u32>,
    number_of_instances: Option<u32>,
    series: Option<Vec<ImagingStudySeries>>,
}

impl ImagingStudyBuilder {
    pub fn new(id: impl Into<String>) -> Self {
        ImagingStudyBuilder {
            domain_resource: DomainResource {
                resource: Resource {
                    id: Some(id.into()),
                    ..Default::default()
                },
                ..Default::default()
            },
            ..Default::default()
        }
    }

    pub fn id(mut self, id: impl Into<String>) -> Self {
        self.domain_resource.resource.id = Some(id.into());
        self
    }

    pub fn identifier(mut self, identifier: Vec<Identifier>) -> Self {
        self.identifier = Some(identifier);
        self
    }

    pub fn add_identifier(mut self, identifier: Identifier) -> Self {
        match &mut self.identifier {
            Some(ident) => ident.push(identifier),
            None => self.identifier = Some(vec![identifier]),
        }
        self
    }

    pub fn status(mut self, status: ImagingStudyStatus) -> Self {
        self.status = Some(status);
        self
    }

    pub fn modality(mut self, modality: Vec<CodeableConcept>) -> Self {
        self.modality = Some(modality);
        self
    }

    pub fn add_modality(mut self, modality: CodeableConcept) -> Self {
        match &mut self.modality {
            Some(mods) => mods.push(modality),
            None => self.modality = Some(vec![modality]),
        }
        self
    }

    pub fn r#type(mut self, r#type: CodeableConcept) -> Self {
        self.r#type = Some(r#type);
        self
    }

    pub fn subject(mut self, subject: Reference<Patient>) -> Self {
        self.subject = Some(subject);
        self
    }

    pub fn started(mut self, started: impl Into<String>) -> Self {
        self.started = Some(started.into());
        self
    }

    pub fn based_on(mut self, based_on: Vec<Reference<Practitioner>>) -> Self {
        self.based_on = Some(based_on);
        self
    }

    pub fn add_based_on(mut self, based_on: Reference<Practitioner>) -> Self {
        match &mut self.based_on {
            Some(based) => based.push(based_on),
            None => self.based_on = Some(vec![based_on]),
        }
        self
    }

    pub fn referrer(mut self, referrer: Reference<Practitioner>) -> Self {
        self.referrer = Some(referrer);
        self
    }

    pub fn interpreter(mut self, interpreter: Vec<Reference<Practitioner>>) -> Self {
        self.interpreter = Some(interpreter);
        self
    }

    pub fn add_interpreter(mut self, interpreter: Reference<Practitioner>) -> Self {
        match &mut self.interpreter {
            Some(ints) => ints.push(interpreter),
            None => self.interpreter = Some(vec![interpreter]),
        }
        self
    }

    pub fn endpoint(mut self, endpoint: Vec<Reference<Endpoint>>) -> Self {
        self.endpoint = Some(endpoint);
        self
    }

    pub fn add_endpoint(mut self, endpoint: Reference<Endpoint>) -> Self {
        match &mut self.endpoint {
            Some(eps) => eps.push(endpoint),
            None => self.endpoint = Some(vec![endpoint]),
        }
        self
    }

    pub fn number_of_series(mut self, number: u32) -> Self {
        self.number_of_series = Some(number);
        self
    }

    pub fn number_of_instances(mut self, number: u32) -> Self {
        self.number_of_instances = Some(number);
        self
    }

    pub fn series(mut self, series: Vec<ImagingStudySeries>) -> Self {
        self.series = Some(series);
        self
    }

    pub fn add_series(mut self, series: ImagingStudySeries) -> Self {
        match &mut self.series {
            Some(s) => s.push(series),
            None => self.series = Some(vec![series]),
        }
        self
    }

    pub fn build(self) -> ImagingStudy {
        ImagingStudy {
            domain_resource: self.domain_resource,
            identifier: self.identifier,
            status: self.status,
            modality: self.modality,
            r#type: self.r#type,
            subject: self.subject,
            started: self.started,
            based_on: self.based_on,
            referrer: self.referrer,
            interpreter: self.interpreter,
            endpoint: self.endpoint,
            number_of_series: self.number_of_series,
            number_of_instances: self.number_of_instances,
            series: self.series,
            resource_type: ImagingStudy::get_resource_type(),
        }
    }
}

#[cfg(test)]
mod test {
    use crate::r4::elements::{CodeableConceptBuilder, Element, ReferenceBuilder};

    use super::*;

    #[test]
    pub fn test_build_imaging_study_should_succeed() {
        let expected = ImagingStudy {
            domain_resource: DomainResource {
                resource: Resource {
                    id: Some("study-1".to_string()),
                    ..Default::default()
                },
                ..Default::default()
            },
            identifier: None,
            status: Some(ImagingStudyStatus::Available),
            modality: None,
            r#type: None,
            subject: None,
            started: None,
            based_on: None,
            referrer: None,
            interpreter: None,
            endpoint: None,
            number_of_series: Some(1),
            number_of_instances: Some(10),
            series: None,
            resource_type: ImagingStudy::get_resource_type(),
        };

        let actual = ImagingStudyBuilder::new("study-1")
            .status(ImagingStudyStatus::Available)
            .number_of_series(1)
            .number_of_instances(10)
            .build();

        assert_eq!(expected, actual);
    }

    #[test]
    pub fn test_build_imaging_study_series_should_succeed() {
        let modality = CodeableConceptBuilder::default()
            .with_text("CT")
            .build();

        let expected = ImagingStudySeries {
            backbone_element: BackboneElement {
                element: Element {
                    id: Some("series-1".to_string()),
                    extention: None,
                },
                modifier_extensions: None,
            },
            uid: "urn:oid:1.2.3.4".to_string(),
            number: Some(1),
            modality: modality.clone(),
            description: Some("CT Head".to_string()),
            number_of_instances: Some(10),
            endpoint: None,
            body_site: None,
            laterality: None,
            started: None,
            instance: None,
        };

        let actual = ImagingStudySeriesBuilder::new("series-1")
            .uid("urn:oid:1.2.3.4")
            .number(1)
            .modality(modality)
            .description("CT Head")
            .number_of_instances(10)
            .build();

        assert_eq!(expected, actual);
    }

    #[test]
    pub fn test_build_imaging_study_instance_should_succeed() {
        let expected = ImagingStudyInstance {
            backbone_element: BackboneElement {
                element: Element {
                    id: Some("instance-1".to_string()),
                    extention: None,
                },
                modifier_extensions: None,
            },
            uid: "urn:oid:1.2.3.4.5".to_string(),
            number: Some(1),
            sop_class: Some("urn:oid:1.2.840.10008.5.1.4.1.1.2".to_string()),
            title: Some("CT Image 1".to_string()),
        };

        let actual = ImagingStudyInstanceBuilder::new("instance-1")
            .uid("urn:oid:1.2.3.4.5")
            .number(1)
            .sop_class("urn:oid:1.2.840.10008.5.1.4.1.1.2")
            .title("CT Image 1")
            .build();

        assert_eq!(expected, actual);
    }

    #[test]
    pub fn test_build_imaging_study_with_all_fields_should_succeed() {
        let subject = ReferenceBuilder::default()
            .with_reference("Patient/patient-1")
            .build::<Patient>();
        let referrer = ReferenceBuilder::default()
            .with_reference("Practitioner/practitioner-1")
            .build::<Practitioner>();
        let interpreter = ReferenceBuilder::default()
            .with_reference("Practitioner/practitioner-2")
            .build::<Practitioner>();
        let endpoint = ReferenceBuilder::default()
            .with_reference("Endpoint/endpoint-1")
            .build::<Endpoint>();

        let modality = CodeableConceptBuilder::default()
            .with_text("CT")
            .build();

        let study_type = CodeableConceptBuilder::default()
            .with_text("CT of head")
            .build();

        let series_modality = CodeableConceptBuilder::default()
            .with_text("CT")
            .build();

        let body_site = CodeableConceptBuilder::default()
            .with_text("Head")
            .build();

        let instance = ImagingStudyInstanceBuilder::new("instance-1")
            .uid("urn:oid:1.2.3.4.5")
            .number(1)
            .sop_class("urn:oid:1.2.840.10008.5.1.4.1.1.2")
            .title("CT Image 1")
            .build();

        let series = ImagingStudySeriesBuilder::new("series-1")
            .uid("urn:oid:1.2.3.4")
            .number(1)
            .modality(series_modality)
            .description("CT Head")
            .number_of_instances(1)
            .body_site(body_site)
            .add_instance(instance)
            .build();

        let identifier = Identifier {
            r#use: Some("official".to_string()),
            system: Some("urn:dicom:uid".to_string()),
            value: Some("urn:oid:1.2.840.113619.2.55.3".to_string()),
            ..Default::default()
        };

        let expected = ImagingStudy {
            domain_resource: DomainResource {
                resource: Resource {
                    id: Some("study-full".to_string()),
                    ..Default::default()
                },
                ..Default::default()
            },
            identifier: Some(vec![identifier.clone()]),
            status: Some(ImagingStudyStatus::Available),
            modality: Some(vec![modality.clone()]),
            r#type: Some(study_type.clone()),
            subject: Some(subject.clone()),
            started: Some("2023-01-15T10:30:00Z".to_string()),
            based_on: None,
            referrer: Some(referrer.clone()),
            interpreter: Some(vec![interpreter.clone()]),
            endpoint: Some(vec![endpoint.clone()]),
            number_of_series: Some(1),
            number_of_instances: Some(1),
            series: Some(vec![series.clone()]),
            resource_type: ImagingStudy::get_resource_type(),
        };

        let actual = ImagingStudyBuilder::new("study-full")
            .add_identifier(identifier.clone())
            .status(ImagingStudyStatus::Available)
            .add_modality(modality.clone())
            .r#type(study_type.clone())
            .subject(subject.clone())
            .started("2023-01-15T10:30:00Z")
            .referrer(referrer.clone())
            .add_interpreter(interpreter.clone())
            .add_endpoint(endpoint.clone())
            .number_of_series(1)
            .number_of_instances(1)
            .add_series(series.clone())
            .build();

        assert_eq!(expected, actual);
    }
}
