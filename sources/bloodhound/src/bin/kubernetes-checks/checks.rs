use std::{fs::File, path::Path};

use bloodhound::{
    check_file_not_mode, ensure_file_owner_and_group_root,
    results::{Checker, CheckerMetadata, CheckerResult, Mode, CheckStatus},
};
use libc::{S_IRWXG, S_IRWXO, S_IWGRP, S_IWOTH, S_IXGRP, S_IXOTH, S_IXUSR};
use serde::Deserialize;

// Bottlerocket doesn't use the standard path for most of these files ¯\_(ツ)_/¯
const KUBELET_SERVICE_FILE: &str = "/etc/systemd/system/kubelet.service.d/exec-start.conf";
const KUBELET_KUBECONFIG_FILE: &str = "/etc/kubernetes/kubelet/kubeconfig";
const KUBELET_CLIENT_CA_FILE: &str = "/etc/kubernetes/pki/ca.crt";
const KUBELET_CONF_FILE: &str = "/etc/kubernetes/kubelet/config";

// =>o.o<= =>o.o<= =>o.o<= =>o.o<= =>o.o<= =>o.o<= =>o.o<= =>o.o<= =>o.o<= =>o.o<=

pub struct K8S04010100Checker {}

impl Checker for K8S04010100Checker {
    fn execute(&self) -> CheckerResult {
        let no_x_xw_xw = S_IXUSR | S_IXGRP | S_IWGRP | S_IXOTH | S_IWOTH;
        check_file_not_mode(KUBELET_SERVICE_FILE, no_x_xw_xw)
    }

    fn metadata(&self) -> CheckerMetadata {
        CheckerMetadata {
            title: "Ensure that the kubelet service file permissions are set to 644 or more restrictive".to_string(),
            id: "4.1.1".to_string(),
            level: 1,
            name: "k8s04010100".to_string(),
            mode: Mode::Automatic,
        }
    }
}

// =>o.o<= =>o.o<= =>o.o<= =>o.o<= =>o.o<= =>o.o<= =>o.o<= =>o.o<= =>o.o<= =>o.o<=

pub struct K8S04010200Checker {}

impl Checker for K8S04010200Checker {
    fn execute(&self) -> CheckerResult {
        ensure_file_owner_and_group_root(KUBELET_SERVICE_FILE)
    }

    fn metadata(&self) -> CheckerMetadata {
        CheckerMetadata {
            title: "Ensure that the kubelet service file ownership is set to root:root".to_string(),
            id: "4.1.2".to_string(),
            level: 1,
            name: "k8s04010200".to_string(),
            mode: Mode::Automatic,
        }
    }
}

// =>o.o<= =>o.o<= =>o.o<= =>o.o<= =>o.o<= =>o.o<= =>o.o<= =>o.o<= =>o.o<= =>o.o<=

pub struct K8S04010500Checker {}

impl Checker for K8S04010500Checker {
    fn execute(&self) -> CheckerResult {
        let no_x_xw_xw = S_IXUSR | S_IXGRP | S_IWGRP | S_IXOTH | S_IWOTH;
        check_file_not_mode(KUBELET_KUBECONFIG_FILE, no_x_xw_xw)
    }

    fn metadata(&self) -> CheckerMetadata {
        CheckerMetadata {
            title: "Ensure that the --kubeconfig kubelet.conf file permissions are set to 644 or more restrictive".to_string(),
            id: "4.1.5".to_string(),
            level: 1,
            name: "k8s04010500".to_string(),
            mode: Mode::Automatic,
        }
    }
}

// =>o.o<= =>o.o<= =>o.o<= =>o.o<= =>o.o<= =>o.o<= =>o.o<= =>o.o<= =>o.o<= =>o.o<=

pub struct K8S04010600Checker {}

impl Checker for K8S04010600Checker {
    fn execute(&self) -> CheckerResult {
        ensure_file_owner_and_group_root(KUBELET_KUBECONFIG_FILE)
    }

    fn metadata(&self) -> CheckerMetadata {
        CheckerMetadata {
            title: "Ensure that the --kubeconfig kubelet.conf file ownership is set to root:root"
                .to_string(),
            id: "4.1.6".to_string(),
            level: 1,
            name: "k8s04010600".to_string(),
            mode: Mode::Automatic,
        }
    }
}

// =>o.o<= =>o.o<= =>o.o<= =>o.o<= =>o.o<= =>o.o<= =>o.o<= =>o.o<= =>o.o<= =>o.o<=

pub struct K8S04010700Checker {}

impl Checker for K8S04010700Checker {
    fn execute(&self) -> CheckerResult {
        let no_x_xwr_xwr = S_IXUSR | S_IRWXG | S_IRWXO;
        check_file_not_mode(KUBELET_CLIENT_CA_FILE, no_x_xwr_xwr)
    }

    fn metadata(&self) -> CheckerMetadata {
        CheckerMetadata {
            title: "Ensure that the certificate authorities file permissions are set to 600 or more restrictive".to_string(),
            id: "4.1.7".to_string(),
            level: 1,
            name: "k8s04010700".to_string(),
            mode: Mode::Automatic,
        }
    }
}

// =>o.o<= =>o.o<= =>o.o<= =>o.o<= =>o.o<= =>o.o<= =>o.o<= =>o.o<= =>o.o<= =>o.o<=

pub struct K8S04010800Checker {}

impl Checker for K8S04010800Checker {
    fn execute(&self) -> CheckerResult {
        ensure_file_owner_and_group_root(KUBELET_CLIENT_CA_FILE)
    }

    fn metadata(&self) -> CheckerMetadata {
        CheckerMetadata {
            title:
                "Ensure that the client certificate authorities file ownership is set to root:root"
                    .to_string(),
            id: "4.1.8".to_string(),
            level: 1,
            name: "k8s04010800".to_string(),
            mode: Mode::Automatic,
        }
    }
}


// =>o.o<= =>o.o<= =>o.o<= =>o.o<= =>o.o<= =>o.o<= =>o.o<= =>o.o<= =>o.o<= =>o.o<=

pub struct K8S04010900Checker {}

impl Checker for K8S04010900Checker {
    fn execute(&self) -> CheckerResult {
        let no_x_xwr_xwr = S_IXUSR | S_IRWXG | S_IRWXO;
        check_file_not_mode(KUBELET_CONF_FILE, no_x_xwr_xwr)
    }

    fn metadata(&self) -> CheckerMetadata {
        CheckerMetadata {
            title: "If the kubelet config.yaml configuration file is being used validate permissions set to 600 or more restrictive".to_string(),
            id: "4.1.9".to_string(),
            level: 1,
            name: "k8s04010900".to_string(),
            mode: Mode::Automatic,
        }
    }
}

// =>o.o<= =>o.o<= =>o.o<= =>o.o<= =>o.o<= =>o.o<= =>o.o<= =>o.o<= =>o.o<= =>o.o<=

pub struct K8S04011000Checker {}

impl Checker for K8S04011000Checker {
    fn execute(&self) -> CheckerResult {
        ensure_file_owner_and_group_root(KUBELET_CONF_FILE)
    }

    fn metadata(&self) -> CheckerMetadata {
        CheckerMetadata {
            title: "If the kubelet config.yaml configuration file is being used validate file ownership is set to root:root"
                .to_string(),
            id: "4.1.10".to_string(),
            level: 1,
            name: "k8s04011000".to_string(),
            mode: Mode::Automatic,
        }
    }
}

// =>o.o<= =>o.o<= =>o.o<= =>o.o<= =>o.o<= =>o.o<= =>o.o<= =>o.o<= =>o.o<= =>o.o<=

pub struct K8S04020100Checker {}

impl Checker for K8S04020100Checker {
    fn execute(&self) -> CheckerResult {
        #[derive(Deserialize)]
        struct Anonymous {
            enabled: bool,
        }

        #[derive(Deserialize)]
        struct Authentication {
            anonymous: Anonymous,
        }

        #[derive(Deserialize)]
        struct KubeletConfig {
            authentication: Authentication,
        }

        let mut result = CheckerResult::default();

        if let Ok(kubelet_file) = File::open(KUBELET_CONF_FILE) {
            if let Ok(config) = serde_yaml::from_reader::<_, KubeletConfig>(kubelet_file) {
                if config.authentication.anonymous.enabled {
                    result.error = "anonymous authentication is configured".to_string();
                    result.status = CheckStatus::FAIL;
                } else {
                    result.status = CheckStatus::PASS;
                }
            } else {
                result.error = "unable to parse kubelet config".to_string()
            }
        } else {
            result.error = format!("unable to read '{}'", KUBELET_CONF_FILE);
        }

        result
    }

    fn metadata(&self) -> CheckerMetadata {
        CheckerMetadata {
            title: "Ensure that the --anonymous-auth argument is set to false".to_string(),
            id: "4.2.1".to_string(),
            level: 1,
            name: "k8s04020100".to_string(),
            mode: Mode::Automatic,
        }
    }
}

// =>o.o<= =>o.o<= =>o.o<= =>o.o<= =>o.o<= =>o.o<= =>o.o<= =>o.o<= =>o.o<= =>o.o<=

pub struct K8S04020200Checker {}

impl Checker for K8S04020200Checker {
    fn execute(&self) -> CheckerResult {
        #[derive(Deserialize)]
        struct Authorization {
            mode: String,
        }

        #[derive(Deserialize)]
        struct KubeletConfig {
            authorization: Authorization,
        }

        let mut result = CheckerResult::default();

        if let Ok(kubelet_file) = File::open(KUBELET_CONF_FILE) {
            if let Ok(config) = serde_yaml::from_reader::<_, KubeletConfig>(kubelet_file) {
                if config.authorization.mode == "AlwaysAllow" {
                    result.error = "AlwaysAllow authorization is configured".to_string();
                    result.status = CheckStatus::FAIL;
                } else {
                    result.status = CheckStatus::PASS;
                }
            } else {
                result.error = "unable to parse kubelet config".to_string()
            }
        } else {
            result.error = format!("unable to read '{}'", KUBELET_CONF_FILE);
        }

        result
    }

    fn metadata(&self) -> CheckerMetadata {
        CheckerMetadata {
            title: "Ensure that the --authorization-mode argument is not set to AlwaysAllow"
                .to_string(),
            id: "4.2.2".to_string(),
            level: 1,
            name: "k8s04020200".to_string(),
            mode: Mode::Automatic,
        }
    }
}

// =>o.o<= =>o.o<= =>o.o<= =>o.o<= =>o.o<= =>o.o<= =>o.o<= =>o.o<= =>o.o<= =>o.o<=

pub struct K8S04020300Checker {}

impl Checker for K8S04020300Checker {
    fn execute(&self) -> CheckerResult {
        #[derive(Deserialize)]
        struct X509 {
            #[serde(rename = "clientCAFile")]
            client_ca_file: String,
        }

        #[derive(Deserialize)]
        struct Authentication {
            x509: X509,
        }

        #[derive(Deserialize)]
        struct KubeletConfig {
            authentication: Authentication,
        }

        let mut result = CheckerResult::default();

        if let Ok(kubelet_file) = File::open(KUBELET_CONF_FILE) {
            if let Ok(config) = serde_yaml::from_reader::<_, KubeletConfig>(kubelet_file) {
                if !config.authentication.x509.client_ca_file.is_empty()
                    && Path::new(&config.authentication.x509.client_ca_file).exists()
                {
                    result.status = CheckStatus::PASS;
                } else {
                    result.error = "CA file not set to expected path".to_string();
                    result.status = CheckStatus::FAIL;
                }
            } else {
                result.error = "unable to parse kubelet config".to_string()
            }
        } else {
            result.error = format!("unable to read '{}'", KUBELET_CONF_FILE);
        }

        result
    }

    fn metadata(&self) -> CheckerMetadata {
        CheckerMetadata {
            title: "Ensure that the --client-ca-file argument is set as appropriate".to_string(),
            id: "4.2.3".to_string(),
            level: 1,
            name: "k8s04020300".to_string(),
            mode: Mode::Automatic,
        }
    }
}

// =>o.o<= =>o.o<= =>o.o<= =>o.o<= =>o.o<= =>o.o<= =>o.o<= =>o.o<= =>o.o<= =>o.o<=

pub struct K8S04020400Checker {}

impl Checker for K8S04020400Checker {
    fn execute(&self) -> CheckerResult {
        #[derive(Deserialize)]
        struct KubeletConfig {
            #[serde(rename = "readOnlyPort")]
            read_only_port: i32,
        }

        let mut result = CheckerResult::default();

        if let Ok(kubelet_file) = File::open(KUBELET_CONF_FILE) {
            if let Ok(config) = serde_yaml::from_reader::<_, KubeletConfig>(kubelet_file) {
                if config.read_only_port != 0 {
                    result.error = "Kubelet readOnlyPort not set to 0".to_string();
                    result.status = CheckStatus::FAIL;
                } else {
                    result.status = CheckStatus::PASS;
                }
            } else {
                result.error = "unable to parse kubelet config".to_string()
            }
        } else {
            result.error = format!("unable to read '{}'", KUBELET_CONF_FILE);
        }

        result
    }

    fn metadata(&self) -> CheckerMetadata {
        CheckerMetadata {
            title: "Verify that the --read-only-port argument is set to 0".to_string(),
            id: "4.2.4".to_string(),
            level: 1,
            name: "k8s04020400".to_string(),
            mode: Mode::Automatic,
        }
    }
}

// =>o.o<= =>o.o<= =>o.o<= =>o.o<= =>o.o<= =>o.o<= =>o.o<= =>o.o<= =>o.o<= =>o.o<=

pub struct K8S04020500Checker {}

impl Checker for K8S04020500Checker {
    fn execute(&self) -> CheckerResult {
        #[derive(Deserialize)]
        struct KubeletConfig {
            #[serde(rename = "streamingConnectionIdleTimeout")]
            streaming_connection_idle_timeout: i32,
        }

        let mut result = CheckerResult::default();

        if let Ok(kubelet_file) = File::open(KUBELET_CONF_FILE) {
            if let Ok(config) = serde_yaml::from_reader::<_, KubeletConfig>(kubelet_file) {
                if config.streaming_connection_idle_timeout == 0 {
                    result.error = "Kubelet streamingConnectionIdleTimeout is set to 0".to_string();
                    result.status = CheckStatus::FAIL;
                } else {
                    result.status = CheckStatus::PASS;
                }
            } else {
                // Normally this value should not be present in the config file, so deserialization is expected to fail.
                result.status = CheckStatus::PASS;
            }
        } else {
            result.error = format!("unable to read '{}'", KUBELET_CONF_FILE);
        }

        result
    }

    fn metadata(&self) -> CheckerMetadata {
        CheckerMetadata {
            title: "Ensure that the --streaming-connection-idle-timeout argument is not set to 0"
                .to_string(),
            id: "4.2.5".to_string(),
            level: 1,
            name: "k8s04020500".to_string(),
            mode: Mode::Automatic,
        }
    }
}
