#[derive(Eq, PartialEq, Debug)]
pub struct DeployResourceResponse {
    pub key: i64,
    pub deployments: Vec<Option<Metadata>>,
}

#[derive(Eq, PartialEq, Debug)]
pub enum Metadata {
    Process(ProcessMetadata),
    Decision(DecisionMetadata),
    DecisionRequirements(DecisionRequirementsMetadata),
}

#[derive(Eq, PartialEq, Debug)]
pub struct ProcessMetadata {
    pub bpmn_process_id: String,
    pub version: i32,
    pub process_definition_key: i64,
    pub resource_name: String,
}

#[derive(Eq, PartialEq, Debug)]
pub struct DecisionMetadata {
    pub dmn_decision_id: String,
    pub dmn_decision_name: String,
    pub version: i32,
    pub decision_key: i64,
    pub dmn_decision_requirements_id: String,
    pub decision_requirements_key: i64,
}

#[derive(Eq, PartialEq, Debug)]
pub struct DecisionRequirementsMetadata {
    pub dmn_decision_requirements_id: String,
    pub dmn_decision_requirements_name: String,
    pub version: i32,
    pub decision_requirements_key: i64,
    pub resource_name: String,
}
