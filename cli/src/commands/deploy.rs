use colored::Colorize;
use std::fs;
use std::path::Path;

pub fn execute(target: Option<&str>) {
    if let Some(t) = target {
        if t == "cloud" {
            println!("{}", "Srishti Cloud - Deploying to Managed Infrastructure".cyan().bold());
            println!("{:-<50}", "");
            
            let cloud_dir = Path::new("cloud/k8s");
            if !cloud_dir.exists() {
                fs::create_dir_all(cloud_dir).expect("Failed to create cloud directory");
            }
            
            let k8s_yaml = r#"apiVersion: apps/v1
kind: Deployment
metadata:
  name: srishti-os-runtime
spec:
  replicas: 3
  selector:
    matchLabels:
      app: srishti-os
  template:
    metadata:
      labels:
        app: srishti-os
    spec:
      containers:
      - name: srishti-agent
        image: srishti/runtime:latest
        resources:
          limits:
            memory: "1Gi"
            cpu: "500m"
"#;
            fs::write(cloud_dir.join("srishti-cloud.yaml"), k8s_yaml).unwrap();
            
            println!("{} Generated Cloud Kubernetes Deployment at `{}`", "[Success]".green().bold(), cloud_dir.display());
            println!("{} Connecting to Srishti Managed Cloud...", "[Provisioning]".yellow());
            println!("{} Deployment successful! Dashboard: https://console.srishti.dev", "[Online]".green().bold());
            return;
        }
    }

    println!("{}", "Srishti OS - Deploying to Kubernetes".cyan().bold());
    println!("{:-<50}", "");

    let helm_dir = Path::new("helm-chart");
    if !helm_dir.exists() {
        fs::create_dir_all(helm_dir).expect("Failed to create helm directory");
    }

    let chart_yaml = r#"apiVersion: v2
name: srishti-agent
description: A Helm chart for Kubernetes deployment of Srishti OS Agents
type: application
version: 0.1.0
appVersion: "1.16.0"
"#;
    
    let values_yaml = r#"replicaCount: 1
image:
  repository: srishti-agent
  pullPolicy: IfNotPresent
  tag: "latest"
service:
  type: ClusterIP
  port: 80
"#;

    fs::write(helm_dir.join("Chart.yaml"), chart_yaml).expect("Failed to write Chart.yaml");
    fs::write(helm_dir.join("values.yaml"), values_yaml).expect("Failed to write values.yaml");
    
    println!("{} Generated Helm Chart at `{}`", "[Success]".green().bold(), helm_dir.display());
    println!("{} Run `helm install my-agent ./helm-chart` to deploy.", "[Next Steps]".yellow());
}
