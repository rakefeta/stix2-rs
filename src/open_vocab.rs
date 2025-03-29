// TODO: use `#[serde(rename_all = "kebab-case")]` to improve readability

#![allow(non_camel_case_types)]

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum OpenVocab {
    AccountTypeOv(AccountTypeOv),
    AttackMotivationOv(AttackMotivationOv),
    AttackResourceLevelOv(AttackResourceLevelOv),
    EncryptionAlgorithmEnum(EncryptionAlgorithmEnum),
    ExtensionTypeEnum(ExtensionTypeEnum),
    GroupingContextOv(GroupingContextOv),
    HashAlgorithmOv(HashAlgorithmOv),
    IdentityClassOv(IdentityClassOv),
    ImplementationLanguageOv(ImplementationLanguageOv),
    IndicatorTypeOv(IndicatorTypeOv),
    IndustrySectorOv(IndustrySectorOv),
    MalwareResultOv(MalwareResultOv),
    MalwareCapabilitiesOv(MalwareCapabilitiesOv),
    MalwareTypeOv(MalwareTypeOv),
    NetworkSocketAddressFamilyEnum(NetworkSocketAddressFamilyEnum),
    NetworkSocketTypeEnum(NetworkSocketTypeEnum),
    OpinionEnum(OpinionEnum),
    PatternTypeOv(PatternTypeOv),
    ProcessorArchitectureOv(ProcessorArchitectureOv),
    RegionOv(RegionOv),
    ReportTypeOv(ReportTypeOv),
    ThreatActorTypeOv(ThreatActorTypeOv),
    ThreatActorRoleOv(ThreatActorRoleOv),
    ThreatActorSophisticationOv(ThreatActorSophisticationOv),
    ToolTypeOv(ToolTypeOv),
    WindowsIntegrityLevelEnum(WindowsIntegrityLevelEnum),
    WindowsPebinaryTypeOv(WindowsPebinaryTypeOv),
    WindowsRegistryDatatypeEnum(WindowsRegistryDatatypeEnum),
    WindowsServiceStartTypeEnum(WindowsServiceStartTypeEnum),
    WindowsServiceTypeEnum(WindowsServiceTypeEnum),
    WindowsServiceStatusEnum(WindowsServiceStatusEnum)
}


#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "kebab-case")]
pub enum AccountTypeOv{
    facebook,
    ldap,
    nis,
    openid,
    radius,
    skype,
    tacacs,
    twitter,
    unix,
    windows_local,
    windows_domain
}


#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum AttackMotivationOv{
    accidental,
    coercion,
    dominance,
    ideology,
    notoriety,
    #[serde(rename = "organizational-gain")]
    organizational_gain,
    #[serde(rename = "personal-gain")]
    personal_gain,
    #[serde(rename = "personal-satisfaction")]
    personal_satisfaction,
    revenge,
    unpredictable
}


#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum AttackResourceLevelOv{
    individual,
    club,
    contest,
    team,
    organization,
    government
}


#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "kebab-case")]
pub enum EncryptionAlgorithmEnum{
    AES_256_GCM,
    #[serde(rename = "ChaCha20-Poly1305")]
    ChaCha20_Poly1305,
    mime_type_indicated
}


#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "kebab-case")]
pub enum ExtensionTypeEnum{
    new_sdo,
    new_sco,
    new_sro,
    property_extension,
    toplevel_property_extension
}


#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all_fields = "kebab-case")]
pub enum GroupingContextOv{
    suspicious_activity,
    malware_analysis,
    unspecified
}


#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "kebab-case")]
pub enum HashAlgorithmOv{
    MD5,
    SHA_1,
    SHA_256,
    SHA_512,
    SHA3_256,
    SHA3_512,
    SSDEEP,
    TLSH
}


#[derive(Serialize, Deserialize, Debug)]
pub enum IdentityClassOv{
    individual,
    group,
    system,
    organization,
    class,
    unknown
}


#[derive(Serialize, Deserialize, Debug)]
pub enum ImplementationLanguageOv {
    bash,
    c,
    #[serde(rename = "c++")]
    cpp,
    #[serde(rename = "c#")]
    csharp,
    go,
    java,
    javascript,
    perl,
    php,
    powershell,
    python,
    ruby,
    rust,
    swift
}


#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "kebab-case")]
pub enum IndicatorTypeOv {
    anomalous_activity,
    anonymization,
    benign,
    compromised,
    malicious_activity,
    attribution,
    unknown
}



// ======== 10.11 Industry Sector Vocabulary ======================
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "kebab-case")]
pub enum Government {
    emergency_services,
    government_local,
    government_national,
    government_public_services,
    government_regional
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Infrastructure {
    dams,
    nuclear,
    water
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "kebab-case")]
pub enum IndustrySectorOv {
    agriculture,
    aerospace,
    automotive,
    chemical,
    commercial,
    communications,
    construction,
    defense,
    education,
    energy,
    entertainment,
    financial_services,
    government(Government),
    healthcare,
    hospitality_leisure,
    infrastructure(Infrastructure),
    insurance,
    manufacturing,
    mining,
    non_profit,
    pharmaceuticals,
    retail,
    technology,
    telecommunications,
    transportation,
    utilities
}

// ======== 10.12 Infrastructure Type Vocabulary ======================

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "kebab-case")]
pub enum InfrastructureTypeOv {
    amplification,
    anonymization,
    botnet,
    command_and_control,
    exfiltration,
    hosting_malware,
    hosting_target_lists,
    phishing,
    reconnaissance,
    staging,
    unknown
}


// ======== 10.13 Malware Result Vocabulary ======================

#[derive(Serialize, Deserialize, Debug)]
pub enum MalwareResultOv {
    malicious,
    suspicious,
    benign,
    unknown
}


// ======== 10.14 Malware Capabilities Vocabulary ======================

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "kebab-case")]
pub enum MalwareCapabilitiesOv {
    accesses_remote_machines,
    anti_debugging,
    anti_disassembly,
    anti_emulation,
    anti_memory_forensics,
    anti_sandbox,
    anti_vm,
    captures_input_peripherals,
    captures_output_peripherals,
    captures_system_state_data,
    cleans_traces_of_infection,
    commits_fraud,
    communicates_with_c2,
    compromises_data_availability,
    compromises_data_integrity,
    compromises_system_availability,
    controls_local_machine,
    degrades_security_software,
    degrades_system_updates,
    determines_c2_server,
    emails_spam,
    escalates_privileges,
    evades_av,
    exfiltrates_data,
    fingerprints_host,
    hides_artifacts,
    hides_executing_code,
    infects_files,
    infects_remote_machines,
    installs_other_components,
    persists_after_system_reboot,
    prevents_artifact_access,
    prevents_artifact_deletion,
    probes_network_environment,
    self_modifies,
    steals_authentication_credentials,
    violates_system_operational_integrity
}


// ======== 10.15 Malware Type Vocabulary ======================

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "kebab-case")]
pub enum MalwareTypeOv {
    adware,
    backdoor,
    bot,
    bootkit,
    ddos,
    downloader,
    dropper,
    exploit_kit,
    keylogger,
    ransomware,
    remote_access_trojan,
    resource_exploitation,
    rogue_security_software,
    rootkit,
    screen_capture,
    spyware,
    trojan,
    unknown,
    virus,
    webshell,
    wiper,
    worm
}


// ======== 10.16 Network Socket Address Family Enumeration ======================

#[derive(Serialize, Deserialize, Debug)]
pub enum NetworkSocketAddressFamilyEnum {
    AF_UNSPEC,
    AF_INET,
    AF_IPX,
    AF_APPLETALK,
    AF_NETBIOS,
    AF_INET6,
    AF_IRDA,
    AF_BTH
}


// ======== 10.17 Network Socket Type Enumeration ======================

#[derive(Serialize, Deserialize, Debug)]
pub enum NetworkSocketTypeEnum {
    SOCK_STREAM,
    AF_ISOCK_DGRAMNET,
    SOCK_RAW,
    SOCK_RDM,
    SOCK_SEQPACKET
}


// ======== 10.18 Opinion Enumeration ======================

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "kebab-case")]
pub enum OpinionEnum {
    strongly_disagree,
    disagree,
    neutral,
    agree,
    strongly_agree
}



// ======== 10.19 Pattern Type Vocabulary ======================

#[derive(Serialize, Deserialize, Debug)]
pub enum PatternTypeOv {
    stix,
    pcre,
    sigma,
    snort,
    suricata, 
    yara
}


// ======== 10.20 Processor Architecture Vocabulary ======================

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "kebab-case")]
pub enum ProcessorArchitectureOv {
    alpha,
    arm,
    ia_64,
    mips,
    powerpc,
    sparc,
    x86,
    x86_64
}


// ======== 10.21 Region Vocabulary ======================

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "kebab-case")]
pub enum africa {
    eastern_africa,
    middle_africa,
    northern_africa,
    southern_africa,
    western_africa
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all_fields = "kebab-case")]
pub enum americas {
    caribbean,
    central_america,
    latin_america_caribbean,
    northern_america,
    south_america
}


#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all_fields = "kebab-case")]
pub enum asia {
    central_asia,
    eastern_asia,
    southern_asia,
    south_eastern_asia,
    western_asia
}


#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "kebab-case")]
pub enum europe {
    eastern_europe,
    northen_europe,
    southern_europe,
    western_europe
}


#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "kebab-case")]
pub enum oceania {
    antarctica,
    australia_new_zealand,
    melanesia,
    micronesia,
    polynesia
}

#[derive(Serialize, Deserialize, Debug)]
pub enum RegionOv {
    africa(africa),
    americas(americas),
    asia(asia),
    europe(europe),
    oceania(oceania)

}


// ======== 10.22 Report Type Vocabulary ======================

#[derive(Serialize, Deserialize, Debug)]
pub enum ReportTypeOv {
    attack_pattern,
    campaign,
    identity,
    indicator,
    intrusion_set,
    malware,
    observed_data,
    threat_actor,
    threat_report,
    tool,
    vulnerability
}


// ======== 10.23 Threat Actor Type Vocabulary ======================

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum ThreatActorTypeOv {
    activist,
    competitor,
    crime_syndicate,
    criminal,
    hacker,
    insider_accidental,
    insider_disgruntled,
    nation_state,
    sensationalist,
    spy,
    terrorist,
    unknown
}


// ======== 10.24 Threat Actor Role Vocabulary ======================

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "kebab-case")]
pub enum ThreatActorRoleOv {
    agent,
    director,
    independent,
    infrastructure_architect,
    infrastructure_operator,
    malware_author,
    sponsor
}


// ======== 10.25 Threat Actor Sophistication Vocabulary ======================

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum ThreatActorSophisticationOv {
    none,
    minimal,
    intermediate,
    advanced,
    expert,
    innovator,
    strategic
}

impl ThreatActorSophisticationOv {
    pub fn iter() -> &'static [ThreatActorSophisticationOv] {
        &[
            ThreatActorSophisticationOv::none,
            ThreatActorSophisticationOv::minimal,
            ThreatActorSophisticationOv::intermediate,
            ThreatActorSophisticationOv::advanced,
            ThreatActorSophisticationOv::expert,
            ThreatActorSophisticationOv::innovator,
            ThreatActorSophisticationOv::strategic,
        ]
    }
}


// ======== 10.26 Tool Type Vocabulary ======================

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "kebab-case")]
pub enum ToolTypeOv {
    denial_of_service,
    exploitation,
    information_gathering,
    network_capture,
    credential_exploitation,
    remote_access,
    vulnerability_scanning,
    unknown
}


// ======== 10.27 Windows™ Integrity Level Enumeration ======================

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "kebab-case")]
pub enum WindowsIntegrityLevelEnum {
    low,
    medium,
    high,
    system
}


// ======== 10.28 Windows™ PE Binary Vocabulary ======================

#[derive(Serialize, Deserialize, Debug)]
pub enum WindowsPebinaryTypeOv {
    dll,
    exe,
    sys
}


// ======== 10.29 Windows™ Registry Datatype Enumeration ======================

#[derive(Serialize, Deserialize, Debug)]
pub enum WindowsRegistryDatatypeEnum {
    REG_NONE,
    REG_SZ,
    REG_EXPAND_SZ,
    REG_BINARY,
    REG_DWORD,
    REG_DWORD_BIG_ENDIAN,
    REG_DWORD_LITTLE_ENDIAN,
    REG_LINK,
    REG_MULTI_SZ,
    REG_RESOURCE_LIST,
    REG_FULL_RESOURCE_DESCRIPTION,
    REG_RESOURCE_REQUIREMENTS_LIST,
    REG_QWORD,
    REG_INVALID_TYPE
}


// ======== 10.30 Windows™ Service Start Type Enumeration ======================

#[derive(Serialize, Deserialize, Debug)]
pub enum WindowsServiceStartTypeEnum {
    SERVICE_AUTO_START,
    SERVICE_BOOT_START,
    SERVICE_DEMAND_START,
    SERVICE_DISABLED,
    SERVICE_SYSTEM_ALERT
}


// ======== 10.31 Windows™ Service Type Enumeration ======================

#[derive(Serialize, Deserialize, Debug)]
pub enum WindowsServiceTypeEnum {
    SERVICE_KERNEL_DRIVER,
    SERVICE_FILE_SYSTEM_DRIVER,
    SERVICE_WIN32_OWN_PROCESS,
    SERVICE_WIN32_SHARE_PROCESS
}


// ======== 10.32 Windows™ Service Status Enumeration ======================

#[derive(Serialize, Deserialize, Debug)]
pub enum WindowsServiceStatusEnum {
    SERVICE_CONTINUE_PENDING,
    SERVICE_PAUSE_PENDING,
    SERVICE_PAUSED,
    SERVICE_RUNNING,
    SERVICE_START_PENDING,
    SERVICE_STOP_PENDING,
    SERVICE_STOPPED
}
