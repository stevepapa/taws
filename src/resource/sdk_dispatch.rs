//! SDK Dispatcher - Single point of AWS SDK invocation
//!
//! This module contains the ONLY concrete SDK implementation code.
//! All other code is data-driven from resources.json.
//!
//! To add support for a new AWS API operation:
//! 1. Add the operation to resources.json
//! 2. Add ONE match arm in invoke_sdk() below

use crate::aws::client::AwsClients;
use anyhow::{anyhow, Result};
use serde_json::{json, Value};

// =============================================================================
// Helper Functions
// =============================================================================

/// Extract a single string parameter from Value, handling both String and Array variants.
/// This is needed because filters are passed as arrays, but some SDK methods expect single strings.
fn extract_param(params: &Value, key: &str) -> String {
    params.get(key)
        .and_then(|v| {
            v.as_str().map(|s| s.to_string())
             .or_else(|| v.as_array().and_then(|a| a.first()).and_then(|v| v.as_str()).map(|s| s.to_string()))
        })
        .unwrap_or_default()
}

// =============================================================================
// Action Functions (write operations)
// =============================================================================

/// Execute an action on a resource (start, stop, terminate, etc.)
pub async fn execute_action(
    service: &str,
    action: &str,
    clients: &AwsClients,
    resource_id: &str,
) -> Result<()> {
    match (service, action) {
        // =====================================================================
        // EC2 Instance Actions
        // =====================================================================
        ("ec2", "start_instance") => {
            clients.ec2.start_instances().instance_ids(resource_id).send().await?;
            Ok(())
        }
        ("ec2", "stop_instance") => {
            clients.ec2.stop_instances().instance_ids(resource_id).send().await?;
            Ok(())
        }
        ("ec2", "terminate_instance") => {
            clients.ec2.terminate_instances().instance_ids(resource_id).send().await?;
            Ok(())
        }

        // =====================================================================
        // Lambda Actions
        // =====================================================================
        ("lambda", "invoke_function") => {
            clients.lambda.invoke()
                .function_name(resource_id)
                .send().await?;
            Ok(())
        }
        ("lambda", "delete_function") => {
            clients.lambda.delete_function()
                .function_name(resource_id)
                .send().await?;
            Ok(())
        }

        // =====================================================================
        // RDS Actions
        // =====================================================================
        ("rds", "start_db_instance") => {
            clients.rds.start_db_instance()
                .db_instance_identifier(resource_id)
                .send().await?;
            Ok(())
        }
        ("rds", "stop_db_instance") => {
            clients.rds.stop_db_instance()
                .db_instance_identifier(resource_id)
                .send().await?;
            Ok(())
        }
        ("rds", "reboot_db_instance") => {
            clients.rds.reboot_db_instance()
                .db_instance_identifier(resource_id)
                .send().await?;
            Ok(())
        }
        ("rds", "delete_db_instance") => {
            clients.rds.delete_db_instance()
                .db_instance_identifier(resource_id)
                .skip_final_snapshot(true)
                .send().await?;
            Ok(())
        }
        ("rds", "delete_db_snapshot") => {
            clients.rds.delete_db_snapshot()
                .db_snapshot_identifier(resource_id)
                .send().await?;
            Ok(())
        }

        // =====================================================================
        // ECS Actions
        // =====================================================================
        ("ecs", "delete_cluster") => {
            clients.ecs.delete_cluster()
                .cluster(resource_id)
                .send().await?;
            Ok(())
        }
        ("ecs", "delete_service") => {
            // resource_id is service ARN which contains cluster info
            // Format: arn:aws:ecs:region:account:service/cluster-name/service-name
            let parts: Vec<&str> = resource_id.split('/').collect();
            if parts.len() >= 2 {
                let cluster = parts[parts.len() - 2];
                clients.ecs.delete_service()
                    .cluster(cluster)
                    .service(resource_id)
                    .force(true)
                    .send().await?;
            }
            Ok(())
        }
        ("ecs", "stop_task") => {
            // resource_id is task ARN which contains cluster info
            let parts: Vec<&str> = resource_id.split('/').collect();
            if parts.len() >= 2 {
                let cluster = parts[parts.len() - 2];
                clients.ecs.stop_task()
                    .cluster(cluster)
                    .task(resource_id)
                    .send().await?;
            }
            Ok(())
        }

        // =====================================================================
        // EKS Actions
        // =====================================================================
        ("eks", "delete_cluster") => {
            clients.eks.delete_cluster()
                .name(resource_id)
                .send().await?;
            Ok(())
        }

        // =====================================================================
        // S3 Actions
        // =====================================================================
        ("s3", "delete_bucket") => {
            clients.s3.delete_bucket()
                .bucket(resource_id)
                .send().await?;
            Ok(())
        }

        // =====================================================================
        // DynamoDB Actions
        // =====================================================================
        ("dynamodb", "delete_table") => {
            clients.dynamodb.delete_table()
                .table_name(resource_id)
                .send().await?;
            Ok(())
        }

        // =====================================================================
        // SQS Actions
        // =====================================================================
        ("sqs", "purge_queue") => {
            clients.sqs.purge_queue()
                .queue_url(resource_id)
                .send().await?;
            Ok(())
        }
        ("sqs", "delete_queue") => {
            clients.sqs.delete_queue()
                .queue_url(resource_id)
                .send().await?;
            Ok(())
        }

        // =====================================================================
        // SNS Actions
        // =====================================================================
        ("sns", "delete_topic") => {
            clients.sns.delete_topic()
                .topic_arn(resource_id)
                .send().await?;
            Ok(())
        }

        // =====================================================================
        // CloudFormation Actions
        // =====================================================================
        ("cloudformation", "delete_stack") => {
            clients.cloudformation.delete_stack()
                .stack_name(resource_id)
                .send().await?;
            Ok(())
        }

        // =====================================================================
        // Secrets Manager Actions
        // =====================================================================
        ("secretsmanager", "rotate_secret") => {
            clients.secretsmanager.rotate_secret()
                .secret_id(resource_id)
                .send().await?;
            Ok(())
        }
        ("secretsmanager", "delete_secret") => {
            clients.secretsmanager.delete_secret()
                .secret_id(resource_id)
                .force_delete_without_recovery(true)
                .send().await?;
            Ok(())
        }

        // =====================================================================
        // Auto Scaling Actions
        // =====================================================================
        ("autoscaling", "delete_auto_scaling_group") => {
            clients.autoscaling.delete_auto_scaling_group()
                .auto_scaling_group_name(resource_id)
                .force_delete(true)
                .send().await?;
            Ok(())
        }

        _ => Err(anyhow!("Unknown action: {}.{}", service, action)),
    }
}

// =============================================================================
// List/Describe Functions (read operations)
// =============================================================================

/// Invoke an AWS SDK method and return the response as JSON.
///
/// This is the ONLY function that contains concrete AWS SDK calls.
/// Everything else is driven by the JSON configuration.
///
/// # Arguments
/// * `service` - AWS service name (e.g., "iam", "ec2")
/// * `method` - SDK method name (e.g., "list_users", "describe_instances")
/// * `clients` - AWS clients container
/// * `params` - Optional parameters from JSON config
///
/// # Returns
/// The SDK response serialized as a serde_json::Value
pub async fn invoke_sdk(
    service: &str,
    method: &str,
    clients: &AwsClients,
    params: &Value,
) -> Result<Value> {
    match (service, method) {
        // =====================================================================
        // IAM Operations
        // =====================================================================
        ("iam", "list_users") => {
            let response = clients.iam.list_users().send().await?;
            let users: Vec<Value> = response
                .users()
                .iter()
                .map(|user| {
                    json!({
                        "UserId": user.user_id(),
                        "UserName": user.user_name(),
                        "Arn": user.arn(),
                        "Path": user.path(),
                        "CreateDate": user.create_date().to_string(),
                    })
                })
                .collect();
            Ok(json!({ "users": users }))
        }

        ("iam", "list_roles") => {
            let response = clients.iam.list_roles().send().await?;
            let roles: Vec<Value> = response
                .roles()
                .iter()
                .map(|role| {
                    json!({
                        "RoleId": role.role_id(),
                        "RoleName": role.role_name(),
                        "Arn": role.arn(),
                        "Path": role.path(),
                        "CreateDate": role.create_date().to_string(),
                        "Description": role.description().unwrap_or("-"),
                    })
                })
                .collect();
            Ok(json!({ "roles": roles }))
        }

        ("iam", "list_policies") => {
            // Handle params - check for scope
            let mut request = clients.iam.list_policies();
            
            if let Some(scope) = params.get("scope").and_then(|v| v.as_str()) {
                request = match scope {
                    "Local" => request.scope(aws_sdk_iam::types::PolicyScopeType::Local),
                    "AWS" => request.scope(aws_sdk_iam::types::PolicyScopeType::Aws),
                    "All" => request.scope(aws_sdk_iam::types::PolicyScopeType::All),
                    _ => request,
                };
            }
            
            let response = request.send().await?;
            let policies: Vec<Value> = response
                .policies()
                .iter()
                .map(|policy| {
                    json!({
                        "PolicyId": policy.policy_id().unwrap_or("-"),
                        "PolicyName": policy.policy_name().unwrap_or("-"),
                        "Arn": policy.arn().unwrap_or("-"),
                        "Path": policy.path().unwrap_or("-"),
                        "CreateDate": policy.create_date().map(|d| d.to_string()).unwrap_or_default(),
                        "AttachmentCount": policy.attachment_count().unwrap_or(0),
                        "IsAttachable": if policy.is_attachable() { "Yes" } else { "No" },
                    })
                })
                .collect();
            Ok(json!({ "policies": policies }))
        }

        ("iam", "list_groups") => {
            let response = clients.iam.list_groups().send().await?;
            let groups: Vec<Value> = response
                .groups()
                .iter()
                .map(|group| {
                    json!({
                        "GroupId": group.group_id(),
                        "GroupName": group.group_name(),
                        "Arn": group.arn(),
                        "Path": group.path(),
                        "CreateDate": group.create_date().to_string(),
                    })
                })
                .collect();
            Ok(json!({ "groups": groups }))
        }

        ("iam", "list_attached_user_policies") => {
            let mut request = clients.iam.list_attached_user_policies();
            let user_name = extract_param(params, "user_name");
            if !user_name.is_empty() {
                request = request.user_name(user_name);
            }
            let response = request.send().await?;
            let policies: Vec<Value> = response
                .attached_policies()
                .iter()
                .map(|policy| {
                    json!({
                        "PolicyName": policy.policy_name().unwrap_or("-"),
                        "PolicyArn": policy.policy_arn().unwrap_or("-"),
                    })
                })
                .collect();
            Ok(json!({ "attached_policies": policies }))
        }

        ("iam", "list_groups_for_user") => {
            let mut request = clients.iam.list_groups_for_user();
            let user_name = extract_param(params, "user_name");
            if !user_name.is_empty() {
                request = request.user_name(user_name);
            }
            let response = request.send().await?;
            let groups: Vec<Value> = response
                .groups()
                .iter()
                .map(|group| {
                    json!({
                        "GroupId": group.group_id(),
                        "GroupName": group.group_name(),
                        "Arn": group.arn(),
                    })
                })
                .collect();
            Ok(json!({ "groups": groups }))
        }

        ("iam", "list_access_keys") => {
            let mut request = clients.iam.list_access_keys();
            let user_name = extract_param(params, "user_name");
            if !user_name.is_empty() {
                request = request.user_name(user_name);
            }
            let response = request.send().await?;
            let access_keys: Vec<Value> = response
                .access_key_metadata()
                .iter()
                .map(|key| {
                    json!({
                        "AccessKeyId": key.access_key_id(),
                        "Status": key.status().map(|s| s.as_str()).unwrap_or("-"),
                        "CreateDate": key.create_date().map(|d| d.to_string()).unwrap_or_default(),
                    })
                })
                .collect();
            Ok(json!({ "access_key_metadata": access_keys }))
        }

        ("iam", "list_attached_role_policies") => {
            let mut request = clients.iam.list_attached_role_policies();
            let role_name = extract_param(params, "role_name");
            if !role_name.is_empty() {
                request = request.role_name(role_name);
            }
            let response = request.send().await?;
            let policies: Vec<Value> = response
                .attached_policies()
                .iter()
                .map(|policy| {
                    json!({
                        "PolicyName": policy.policy_name().unwrap_or("-"),
                        "PolicyArn": policy.policy_arn().unwrap_or("-"),
                    })
                })
                .collect();
            Ok(json!({ "attached_policies": policies }))
        }

        ("iam", "get_group") => {
            let mut request = clients.iam.get_group();
            let group_name = extract_param(params, "group_name");
            if !group_name.is_empty() {
                request = request.group_name(group_name);
            }
            let response = request.send().await?;
            let users: Vec<Value> = response
                .users()
                .iter()
                .map(|user| {
                    json!({
                        "UserId": user.user_id(),
                        "UserName": user.user_name(),
                        "Arn": user.arn(),
                    })
                })
                .collect();
            Ok(json!({ "users": users }))
        }

        // =====================================================================
        // EC2 Operations
        // =====================================================================
        ("ec2", "describe_instances") => {
            let response = clients.ec2.describe_instances().send().await?;
            
            // Flatten instances from reservations
            let mut instances: Vec<Value> = Vec::new();
            for reservation in response.reservations() {
                for instance in reservation.instances() {
                    // Extract tags into a map
                    let tags: std::collections::HashMap<String, String> = instance
                        .tags()
                        .iter()
                        .filter_map(|tag| {
                            Some((tag.key()?.to_string(), tag.value()?.to_string()))
                        })
                        .collect();
                    
                    instances.push(json!({
                        "InstanceId": instance.instance_id().unwrap_or("-"),
                        "InstanceType": instance.instance_type().map(|t| t.as_str()).unwrap_or("-"),
                        "State": instance.state().and_then(|s| s.name()).map(|n| n.as_str()).unwrap_or("-"),
                        "AvailabilityZone": instance.placement().and_then(|p| p.availability_zone()).unwrap_or("-"),
                        "PublicIpAddress": instance.public_ip_address().unwrap_or("-"),
                        "PrivateIpAddress": instance.private_ip_address().unwrap_or("-"),
                        "LaunchTime": instance.launch_time().map(|t| t.to_string()).unwrap_or_default(),
                        "Tags": tags,
                    }));
                }
            }
            Ok(json!({ "reservations": instances }))
        }

        ("ec2", "describe_vpcs") => {
            let response = clients.ec2.describe_vpcs().send().await?;
            let vpcs: Vec<Value> = response
                .vpcs()
                .iter()
                .map(|vpc| {
                    let tags: std::collections::HashMap<String, String> = vpc
                        .tags()
                        .iter()
                        .filter_map(|tag| {
                            Some((tag.key()?.to_string(), tag.value()?.to_string()))
                        })
                        .collect();
                    
                    json!({
                        "VpcId": vpc.vpc_id().unwrap_or("-"),
                        "State": vpc.state().map(|s| s.as_str()).unwrap_or("-"),
                        "CidrBlock": vpc.cidr_block().unwrap_or("-"),
                        "IsDefault": if vpc.is_default().unwrap_or(false) { "Yes" } else { "No" },
                        "InstanceTenancy": vpc.instance_tenancy().map(|t| t.as_str()).unwrap_or("-"),
                        "Tags": tags,
                    })
                })
                .collect();
            Ok(json!({ "vpcs": vpcs }))
        }

        ("ec2", "describe_subnets") => {
            let mut request = clients.ec2.describe_subnets();
            
            // Handle VPC filter from params
            if let Some(vpc_ids) = params.get("vpc_ids").and_then(|v| v.as_array()) {
                use aws_sdk_ec2::types::Filter;
                let vpc_id_values: Vec<String> = vpc_ids
                    .iter()
                    .filter_map(|v| v.as_str().map(String::from))
                    .collect();
                let filter = Filter::builder()
                    .name("vpc-id")
                    .set_values(Some(vpc_id_values))
                    .build();
                request = request.filters(filter);
            }
            
            let response = request.send().await?;
            let subnets: Vec<Value> = response
                .subnets()
                .iter()
                .map(|subnet| {
                    let tags: std::collections::HashMap<String, String> = subnet
                        .tags()
                        .iter()
                        .filter_map(|tag| {
                            Some((tag.key()?.to_string(), tag.value()?.to_string()))
                        })
                        .collect();
                    
                    json!({
                        "SubnetId": subnet.subnet_id().unwrap_or("-"),
                        "VpcId": subnet.vpc_id().unwrap_or("-"),
                        "State": subnet.state().map(|s| s.as_str()).unwrap_or("-"),
                        "CidrBlock": subnet.cidr_block().unwrap_or("-"),
                        "AvailabilityZone": subnet.availability_zone().unwrap_or("-"),
                        "AvailableIpAddressCount": subnet.available_ip_address_count().unwrap_or(0),
                        "Tags": tags,
                    })
                })
                .collect();
            Ok(json!({ "subnets": subnets }))
        }

        ("ec2", "describe_security_groups") => {
            let mut request = clients.ec2.describe_security_groups();
            
            // Handle VPC filter from params
            if let Some(vpc_ids) = params.get("vpc_ids").and_then(|v| v.as_array()) {
                use aws_sdk_ec2::types::Filter;
                let vpc_id_values: Vec<String> = vpc_ids
                    .iter()
                    .filter_map(|v| v.as_str().map(String::from))
                    .collect();
                let filter = Filter::builder()
                    .name("vpc-id")
                    .set_values(Some(vpc_id_values))
                    .build();
                request = request.filters(filter);
            }
            
            let response = request.send().await?;
            let groups: Vec<Value> = response
                .security_groups()
                .iter()
                .map(|sg| {
                    json!({
                        "GroupId": sg.group_id().unwrap_or("-"),
                        "GroupName": sg.group_name().unwrap_or("-"),
                        "VpcId": sg.vpc_id().unwrap_or("-"),
                        "Description": sg.description().unwrap_or("-"),
                        "OwnerId": sg.owner_id().unwrap_or("-"),
                    })
                })
                .collect();
            Ok(json!({ "security_groups": groups }))
        }

        // =====================================================================
        // S3 Operations
        // =====================================================================
        ("s3", "list_buckets") => {
            let response = clients.s3.list_buckets().send().await?;
            let buckets: Vec<Value> = response
                .buckets()
                .iter()
                .map(|bucket| {
                    json!({
                        "Name": bucket.name().unwrap_or("-"),
                        "CreationDate": bucket.creation_date().map(|d| d.to_string()).unwrap_or_default(),
                    })
                })
                .collect();
            Ok(json!({ "buckets": buckets }))
        }

        // =====================================================================
        // RDS Operations
        // =====================================================================
        ("rds", "describe_db_instances") => {
            let response = clients.rds.describe_db_instances().send().await?;
            let instances: Vec<Value> = response
                .db_instances()
                .iter()
                .map(|db| {
                    json!({
                        "DBInstanceIdentifier": db.db_instance_identifier().unwrap_or("-"),
                        "DBInstanceStatus": db.db_instance_status().unwrap_or("-"),
                        "Engine": db.engine().unwrap_or("-"),
                        "DBInstanceClass": db.db_instance_class().unwrap_or("-"),
                        "AvailabilityZone": db.availability_zone().unwrap_or("-"),
                        "Endpoint": db.endpoint().and_then(|e| e.address()).unwrap_or("-"),
                    })
                })
                .collect();
            Ok(json!({ "db_instances": instances }))
        }

        ("rds", "describe_db_snapshots") => {
            let db_id = extract_param(params, "db_instance_identifier");
            let mut req = clients.rds.describe_db_snapshots();
            if !db_id.is_empty() {
                req = req.db_instance_identifier(&db_id);
            }
            let response = req.send().await?;
            let snapshots: Vec<Value> = response
                .db_snapshots()
                .iter()
                .map(|snap| {
                    json!({
                        "DBSnapshotIdentifier": snap.db_snapshot_identifier().unwrap_or("-"),
                        "DBInstanceIdentifier": snap.db_instance_identifier().unwrap_or("-"),
                        "Status": snap.status().unwrap_or("-"),
                        "SnapshotType": snap.snapshot_type().unwrap_or("-"),
                        "Engine": snap.engine().unwrap_or("-"),
                        "AllocatedStorage": snap.allocated_storage(),
                        "SnapshotCreateTime": snap.snapshot_create_time().map(|t| t.to_string()).unwrap_or("-".to_string()),
                    })
                })
                .collect();
            Ok(json!({ "db_snapshots": snapshots }))
        }

        // =====================================================================
        // DynamoDB Operations
        // =====================================================================
        ("dynamodb", "list_tables") => {
            let response = clients.dynamodb.list_tables().send().await?;
            let tables: Vec<Value> = response
                .table_names()
                .iter()
                .map(|name| {
                    json!({
                        "TableName": name,
                    })
                })
                .collect();
            Ok(json!({ "table_names": tables }))
        }

        // =====================================================================
        // Lambda Operations
        // =====================================================================
        ("lambda", "list_functions") => {
            let response = clients.lambda.list_functions().send().await?;
            let functions: Vec<Value> = response
                .functions()
                .iter()
                .map(|f| {
                    json!({
                        "FunctionName": f.function_name().unwrap_or("-"),
                        "Runtime": f.runtime().map(|r| r.as_str()).unwrap_or("-"),
                        "MemorySize": f.memory_size().unwrap_or(0),
                        "LastModified": f.last_modified().unwrap_or("-"),
                        "Description": f.description().unwrap_or("-"),
                    })
                })
                .collect();
            Ok(json!({ "functions": functions }))
        }

        // =====================================================================
        // ECS Operations
        // =====================================================================
        ("ecs", "list_clusters_with_details") => {
            // 1. List clusters to get ARNs
            let list_resp = clients.ecs.list_clusters().send().await?;
            let cluster_arns = list_resp.cluster_arns();
            
            if cluster_arns.is_empty() {
                return Ok(json!({ "clusters": [] }));
            }
            
            // 2. Describe clusters to get details
            let desc_resp = clients.ecs.describe_clusters()
                .set_clusters(Some(cluster_arns.to_vec()))
                .send()
                .await?;
                
            let clusters: Vec<Value> = desc_resp
                .clusters()
                .iter()
                .map(|c| {
                    json!({
                        "clusterArn": c.cluster_arn().unwrap_or("-"),
                        "clusterName": c.cluster_name().unwrap_or("-"),
                        "status": c.status().unwrap_or("-"),
                        "runningTasksCount": c.running_tasks_count(),
                        "registeredContainerInstancesCount": c.registered_container_instances_count(),
                    })
                })
                .collect();
            Ok(json!({ "clusters": clusters }))
        }

        ("ecs", "list_services_with_details") => {
            let cluster = extract_param(params, "cluster");
            if cluster.is_empty() {
                return Ok(json!({ "services": [] }));
            }
            
            // 1. List services to get ARNs
            let list_resp = clients.ecs.list_services()
                .cluster(&cluster)
                .send()
                .await?;
            let service_arns = list_resp.service_arns();
            
            if service_arns.is_empty() {
                return Ok(json!({ "services": [] }));
            }
            
            // 2. Describe services to get details
            let desc_resp = clients.ecs.describe_services()
                .cluster(&cluster)
                .set_services(Some(service_arns.to_vec()))
                .send()
                .await?;
                
            let services: Vec<Value> = desc_resp
                .services()
                .iter()
                .map(|s| {
                    json!({
                        "serviceArn": s.service_arn().unwrap_or("-"),
                        "serviceName": s.service_name().unwrap_or("-"),
                        "status": s.status().unwrap_or("-"),
                        "desiredCount": s.desired_count(),
                        "runningCount": s.running_count(),
                        "launchType": s.launch_type().map(|l| l.as_str()).unwrap_or("-"),
                        "clusterArn": s.cluster_arn().unwrap_or("-"),
                    })
                })
                .collect();
            Ok(json!({ "services": services }))
        }

        ("ecs", "list_tasks_with_details") => {
            let cluster = extract_param(params, "cluster");
            if cluster.is_empty() {
                return Ok(json!({ "tasks": [] }));
            }
            
            // 1. List tasks to get ARNs
            let list_resp = clients.ecs.list_tasks()
                .cluster(&cluster)
                .send()
                .await?;
            let task_arns = list_resp.task_arns();
            
            if task_arns.is_empty() {
                return Ok(json!({ "tasks": [] }));
            }
            
            // 2. Describe tasks to get details
            let desc_resp = clients.ecs.describe_tasks()
                .cluster(&cluster)
                .set_tasks(Some(task_arns.to_vec()))
                .send()
                .await?;
                
            let tasks: Vec<Value> = desc_resp
                .tasks()
                .iter()
                .map(|t| {
                    json!({
                        "taskArn": t.task_arn().unwrap_or("-"),
                        "lastStatus": t.last_status().unwrap_or("-"),
                        "desiredStatus": t.desired_status().unwrap_or("-"),
                        "cpu": t.cpu().unwrap_or("-"),
                        "memory": t.memory().unwrap_or("-"),
                        "clusterArn": t.cluster_arn().unwrap_or("-"),
                    })
                })
                .collect();
            Ok(json!({ "tasks": tasks }))
        }

        // =====================================================================
        // SQS Operations
        // =====================================================================
        ("sqs", "list_queues") => {
            let response = clients.sqs.list_queues().send().await?;
            let queues: Vec<Value> = response
                .queue_urls()
                .iter()
                .map(|url| {
                    json!({
                        "QueueUrl": url,
                    })
                })
                .collect();
            Ok(json!({ "queue_urls": queues }))
        }

        // =====================================================================
        // SNS Operations
        // =====================================================================
        ("sns", "list_topics") => {
            let response = clients.sns.list_topics().send().await?;
            let topics: Vec<Value> = response
                .topics()
                .iter()
                .map(|t| {
                    json!({
                        "TopicArn": t.topic_arn().unwrap_or("-"),
                    })
                })
                .collect();
            Ok(json!({ "topics": topics }))
        }

        // =====================================================================
        // CloudFormation Operations
        // =====================================================================
        ("cloudformation", "describe_stacks") => {
            let response = clients.cloudformation.describe_stacks().send().await?;
            let stacks: Vec<Value> = response
                .stacks()
                .iter()
                .map(|stack| {
                    json!({
                        "StackName": stack.stack_name(),
                        "StackId": stack.stack_id().unwrap_or("-"),
                        "StackStatus": stack.stack_status().map(|s| s.as_str()).unwrap_or("-"),
                        "CreationTime": stack.creation_time().map(|t| t.to_string()).unwrap_or_default(),
                        "LastUpdatedTime": stack.last_updated_time().map(|t| t.to_string()).unwrap_or_else(|| "-".to_string()),
                        "Description": stack.description().unwrap_or("-"),
                    })
                })
                .collect();
            Ok(json!({ "stacks": stacks }))
        }

        // =====================================================================
        // CloudWatch Logs Operations
        // =====================================================================
        ("cloudwatchlogs", "describe_log_groups") => {
            let response = clients.logs.describe_log_groups().send().await?;
            let log_groups: Vec<Value> = response
                .log_groups()
                .iter()
                .map(|lg| {
                    json!({
                        "logGroupName": lg.log_group_name().unwrap_or("-"),
                        "logGroupArn": lg.arn().unwrap_or("-"),
                        "storedBytes": lg.stored_bytes().unwrap_or(0),
                        "retentionInDays": lg.retention_in_days().map(|d| d.to_string()).unwrap_or("Never".to_string()),
                        "creationTime": lg.creation_time().map(|t| t.to_string()).unwrap_or_default(),
                    })
                })
                .collect();
            Ok(json!({ "log_groups": log_groups }))
        }

        // =====================================================================
        // Secrets Manager Operations
        // =====================================================================
        ("secretsmanager", "list_secrets") => {
            let response = clients.secretsmanager.list_secrets().send().await?;
            let secrets: Vec<Value> = response
                .secret_list()
                .iter()
                .map(|secret| {
                    json!({
                        "Name": secret.name().unwrap_or("-"),
                        "ARN": secret.arn().unwrap_or("-"),
                        "Description": secret.description().unwrap_or("-"),
                        "LastAccessedDate": secret.last_accessed_date().map(|d| d.to_string()).unwrap_or_else(|| "-".to_string()),
                        "LastChangedDate": secret.last_changed_date().map(|d| d.to_string()).unwrap_or_else(|| "-".to_string()),
                    })
                })
                .collect();
            Ok(json!({ "secrets": secrets }))
        }

        // =====================================================================
        // SSM (Systems Manager) Operations
        // =====================================================================
        ("ssm", "describe_parameters") => {
            let response = clients.ssm.describe_parameters().send().await?;
            let parameters: Vec<Value> = response
                .parameters()
                .iter()
                .map(|param| {
                    json!({
                        "Name": param.name().unwrap_or("-"),
                        "Type": param.r#type().map(|t| t.as_str()).unwrap_or("-"),
                        "Tier": param.tier().map(|t| t.as_str()).unwrap_or("-"),
                        "LastModifiedDate": param.last_modified_date().map(|d| d.to_string()).unwrap_or_else(|| "-".to_string()),
                        "Description": param.description().unwrap_or("-"),
                    })
                })
                .collect();
            Ok(json!({ "parameters": parameters }))
        }

        // =====================================================================
        // EKS Operations
        // =====================================================================
        ("eks", "list_clusters_with_details") => {
            // 1. List clusters to get names
            let list_resp = clients.eks.list_clusters().send().await?;
            let cluster_names = list_resp.clusters();
            
            if cluster_names.is_empty() {
                return Ok(json!({ "clusters": [] }));
            }
            
            // 2. Describe each cluster to get details
            let mut clusters: Vec<Value> = Vec::new();
            for name in cluster_names {
                if let Ok(desc_resp) = clients.eks.describe_cluster().name(name).send().await {
                    if let Some(cluster) = desc_resp.cluster() {
                        clusters.push(json!({
                            "name": cluster.name().unwrap_or("-"),
                            "arn": cluster.arn().unwrap_or("-"),
                            "status": cluster.status().map(|s| s.as_str()).unwrap_or("-"),
                            "version": cluster.version().unwrap_or("-"),
                            "endpoint": cluster.endpoint().unwrap_or("-"),
                        }));
                    }
                }
            }
            Ok(json!({ "clusters": clusters }))
        }

        // =====================================================================
        // API Gateway Operations
        // =====================================================================
        ("apigateway", "get_rest_apis") => {
            let response = clients.apigateway.get_rest_apis().send().await?;
            let items: Vec<Value> = response
                .items()
                .iter()
                .map(|api| {
                    json!({
                        "id": api.id().unwrap_or("-"),
                        "name": api.name().unwrap_or("-"),
                        "description": api.description().unwrap_or("-"),
                        "createdDate": api.created_date().map(|d| d.to_string()).unwrap_or_else(|| "-".to_string()),
                    })
                })
                .collect();
            Ok(json!({ "items": items }))
        }

        // =====================================================================
        // Route53 Operations
        // =====================================================================
        ("route53", "list_hosted_zones") => {
            let response = clients.route53.list_hosted_zones().send().await?;
            let zones: Vec<Value> = response
                .hosted_zones()
                .iter()
                .map(|zone| {
                    let is_private = zone.config()
                        .map(|c| c.private_zone())
                        .unwrap_or(false);
                    json!({
                        "Id": zone.id(),
                        "Name": zone.name(),
                        "ResourceRecordSetCount": zone.resource_record_set_count().unwrap_or(0),
                        "Config.PrivateZone": if is_private { "Private" } else { "Public" },
                    })
                })
                .collect();
            Ok(json!({ "hosted_zones": zones }))
        }

        // =====================================================================
        // ElastiCache Operations
        // =====================================================================
        ("elasticache", "describe_cache_clusters") => {
            let response = clients.elasticache.describe_cache_clusters().send().await?;
            let clusters: Vec<Value> = response
                .cache_clusters()
                .iter()
                .map(|cluster| {
                    json!({
                        "CacheClusterId": cluster.cache_cluster_id().unwrap_or("-"),
                        "CacheClusterStatus": cluster.cache_cluster_status().unwrap_or("-"),
                        "Engine": cluster.engine().unwrap_or("-"),
                        "CacheNodeType": cluster.cache_node_type().unwrap_or("-"),
                        "NumCacheNodes": cluster.num_cache_nodes().unwrap_or(0),
                    })
                })
                .collect();
            Ok(json!({ "cache_clusters": clusters }))
        }

        // =====================================================================
        // ACM Operations
        // =====================================================================
        ("acm", "list_certificates") => {
            let response = clients.acm.list_certificates().send().await?;
            let certs: Vec<Value> = response
                .certificate_summary_list()
                .iter()
                .map(|cert| {
                    json!({
                        "DomainName": cert.domain_name().unwrap_or("-"),
                        "CertificateArn": cert.certificate_arn().unwrap_or("-"),
                        "Status": cert.status().map(|s| s.as_str()).unwrap_or("-"),
                        "Type": cert.r#type().map(|t| t.as_str()).unwrap_or("-"),
                        "InUse": if cert.in_use().unwrap_or(false) { "Yes" } else { "No" },
                    })
                })
                .collect();
            Ok(json!({ "certificates": certs }))
        }

        // =====================================================================
        // Athena Operations
        // =====================================================================
        ("athena", "list_work_groups") => {
            let response = clients.athena.list_work_groups().send().await?;
            let workgroups: Vec<Value> = response
                .work_groups()
                .iter()
                .map(|wg| {
                    json!({
                        "Name": wg.name().unwrap_or("-"),
                        "State": wg.state().map(|s| s.as_str()).unwrap_or("-"),
                        "Description": wg.description().unwrap_or("-"),
                    })
                })
                .collect();
            Ok(json!({ "work_groups": workgroups }))
        }

        // =====================================================================
        // Auto Scaling Operations
        // =====================================================================
        ("autoscaling", "describe_auto_scaling_groups") => {
            let response = clients.autoscaling.describe_auto_scaling_groups().send().await?;
            let groups: Vec<Value> = response
                .auto_scaling_groups()
                .iter()
                .map(|asg| {
                    let azs: Vec<String> = asg.availability_zones().iter().map(|s| s.to_string()).collect();
                    json!({
                        "AutoScalingGroupName": asg.auto_scaling_group_name(),
                        "MinSize": asg.min_size(),
                        "MaxSize": asg.max_size(),
                        "DesiredCapacity": asg.desired_capacity(),
                        "InstanceCount": asg.instances().len(),
                        "AvailabilityZones": azs.join(", "),
                    })
                })
                .collect();
            Ok(json!({ "auto_scaling_groups": groups }))
        }

        // =====================================================================
        // Backup Operations
        // =====================================================================
        ("backup", "list_backup_vaults") => {
            let response = clients.backup.list_backup_vaults().send().await?;
            let vaults: Vec<Value> = response
                .backup_vault_list()
                .iter()
                .map(|vault| {
                    json!({
                        "BackupVaultName": vault.backup_vault_name().unwrap_or("-"),
                        "BackupVaultArn": vault.backup_vault_arn().unwrap_or("-"),
                        "NumberOfRecoveryPoints": vault.number_of_recovery_points(),
                        "CreationDate": vault.creation_date().map(|d| d.to_string()).unwrap_or_else(|| "-".to_string()),
                    })
                })
                .collect();
            Ok(json!({ "backup_vaults": vaults }))
        }

        // =====================================================================
        // Batch Operations
        // =====================================================================
        ("batch", "describe_compute_environments") => {
            let response = clients.batch.describe_compute_environments().send().await?;
            let envs: Vec<Value> = response
                .compute_environments()
                .iter()
                .map(|env| {
                    json!({
                        "computeEnvironmentName": env.compute_environment_name().unwrap_or("-"),
                        "computeEnvironmentArn": env.compute_environment_arn().unwrap_or("-"),
                        "state": env.state().map(|s| s.as_str()).unwrap_or("-"),
                        "status": env.status().map(|s| s.as_str()).unwrap_or("-"),
                        "type": env.r#type().map(|t| t.as_str()).unwrap_or("-"),
                    })
                })
                .collect();
            Ok(json!({ "compute_environments": envs }))
        }

        ("batch", "describe_job_queues") => {
            let response = clients.batch.describe_job_queues().send().await?;
            let queues: Vec<Value> = response
                .job_queues()
                .iter()
                .map(|queue| {
                    json!({
                        "jobQueueName": queue.job_queue_name(),
                        "jobQueueArn": queue.job_queue_arn(),
                        "state": queue.state().map(|s| s.as_str()).unwrap_or("-"),
                        "status": queue.status().map(|s| s.as_str()).unwrap_or("-"),
                        "priority": queue.priority(),
                    })
                })
                .collect();
            Ok(json!({ "job_queues": queues }))
        }

        // =====================================================================
        // Budgets Operations
        // =====================================================================
        ("budgets", "describe_budgets") => {
            // Get account ID from STS
            let sts_response = clients.sts.get_caller_identity().send().await?;
            let account_id = sts_response.account().unwrap_or("-");
            
            let response = clients.budgets.describe_budgets().account_id(account_id).send().await?;
            let budgets: Vec<Value> = response
                .budgets()
                .iter()
                .map(|budget| {
                    let limit = budget.budget_limit()
                        .map(|l| format!("{} {}", l.amount(), l.unit()))
                        .unwrap_or_else(|| "-".to_string());
                    json!({
                        "BudgetName": budget.budget_name(),
                        "BudgetType": budget.budget_type().as_str(),
                        "BudgetLimit": limit,
                    })
                })
                .collect();
            Ok(json!({ "budgets": budgets }))
        }

        // =====================================================================
        // CloudFront Operations
        // =====================================================================
        ("cloudfront", "list_distributions") => {
            let response = clients.cloudfront.list_distributions().send().await?;
            let distributions: Vec<Value> = response
                .distribution_list()
                .map(|list| {
                    list.items()
                        .iter()
                        .map(|dist| {
                            json!({
                                "Id": dist.id(),
                                "DomainName": dist.domain_name(),
                                "Status": dist.status(),
                                "Enabled": if dist.enabled() { "Yes" } else { "No" },
                            })
                        })
                        .collect()
                })
                .unwrap_or_default();
            Ok(json!({ "distributions": distributions }))
        }

        // =====================================================================
        // CloudTrail Operations
        // =====================================================================
        ("cloudtrail", "describe_trails") => {
            let response = clients.cloudtrail.describe_trails().send().await?;
            let trails: Vec<Value> = response
                .trail_list()
                .iter()
                .map(|trail| {
                    json!({
                        "Name": trail.name().unwrap_or("-"),
                        "TrailARN": trail.trail_arn().unwrap_or("-"),
                        "S3BucketName": trail.s3_bucket_name().unwrap_or("-"),
                        "IsMultiRegionTrail": if trail.is_multi_region_trail().unwrap_or(false) { "Yes" } else { "No" },
                        "LogFileValidationEnabled": if trail.log_file_validation_enabled().unwrap_or(false) { "Yes" } else { "No" },
                    })
                })
                .collect();
            Ok(json!({ "trails": trails }))
        }

        // =====================================================================
        // CodeBuild Operations
        // =====================================================================
        ("codebuild", "list_projects_with_details") => {
            let list_response = clients.codebuild.list_projects().send().await?;
            let project_names = list_response.projects();
            
            if project_names.is_empty() {
                return Ok(json!({ "projects": [] }));
            }
            
            let batch_response = clients.codebuild.batch_get_projects()
                .set_names(Some(project_names.to_vec()))
                .send()
                .await?;
            
            let projects: Vec<Value> = batch_response
                .projects()
                .iter()
                .map(|proj| {
                    json!({
                        "name": proj.name().unwrap_or("-"),
                        "sourceType": proj.source().map(|s| s.r#type().as_str()).unwrap_or("-"),
                        "created": proj.created().map(|d| d.to_string()).unwrap_or_else(|| "-".to_string()),
                    })
                })
                .collect();
            Ok(json!({ "projects": projects }))
        }

        // =====================================================================
        // CodePipeline Operations
        // =====================================================================
        ("codepipeline", "list_pipelines") => {
            let response = clients.codepipeline.list_pipelines().send().await?;
            let pipelines: Vec<Value> = response
                .pipelines()
                .iter()
                .map(|pipeline| {
                    json!({
                        "name": pipeline.name().unwrap_or("-"),
                        "version": pipeline.version().unwrap_or(0),
                        "created": pipeline.created().map(|d| d.to_string()).unwrap_or_else(|| "-".to_string()),
                        "updated": pipeline.updated().map(|d| d.to_string()).unwrap_or_else(|| "-".to_string()),
                    })
                })
                .collect();
            Ok(json!({ "pipelines": pipelines }))
        }

        // =====================================================================
        // Cognito Operations
        // =====================================================================
        ("cognitoidentityprovider", "list_user_pools") => {
            let response = clients.cognito_idp.list_user_pools().max_results(60).send().await?;
            let pools: Vec<Value> = response
                .user_pools()
                .iter()
                .map(|pool| {
                    json!({
                        "Id": pool.id().unwrap_or("-"),
                        "Name": pool.name().unwrap_or("-"),
                        "Status": "-",
                        "CreationDate": pool.creation_date().map(|d| d.to_string()).unwrap_or_else(|| "-".to_string()),
                    })
                })
                .collect();
            Ok(json!({ "user_pools": pools }))
        }

        // =====================================================================
        // Config Operations
        // =====================================================================
        ("config", "describe_config_rules") => {
            let response = clients.config.describe_config_rules().send().await?;
            let rules: Vec<Value> = response
                .config_rules()
                .iter()
                .map(|rule| {
                    json!({
                        "ConfigRuleName": rule.config_rule_name().unwrap_or("-"),
                        "ConfigRuleState": rule.config_rule_state().map(|s| s.as_str()).unwrap_or("-"),
                        "Source": rule.source().map(|s| s.owner().as_str()).unwrap_or("-"),
                    })
                })
                .collect();
            Ok(json!({ "config_rules": rules }))
        }

        // =====================================================================
        // Direct Connect Operations
        // =====================================================================
        ("directconnect", "describe_connections") => {
            let response = clients.directconnect.describe_connections().send().await?;
            let connections: Vec<Value> = response
                .connections()
                .iter()
                .map(|conn| {
                    json!({
                        "connectionId": conn.connection_id().unwrap_or("-"),
                        "connectionName": conn.connection_name().unwrap_or("-"),
                        "connectionState": conn.connection_state().map(|s| s.as_str()).unwrap_or("-"),
                        "bandwidth": conn.bandwidth().unwrap_or("-"),
                        "location": conn.location().unwrap_or("-"),
                    })
                })
                .collect();
            Ok(json!({ "connections": connections }))
        }

        // =====================================================================
        // ECR Operations
        // =====================================================================
        ("ecr", "describe_repositories") => {
            let response = clients.ecr.describe_repositories().send().await?;
            let repos: Vec<Value> = response
                .repositories()
                .iter()
                .map(|repo| {
                    json!({
                        "repositoryName": repo.repository_name().unwrap_or("-"),
                        "repositoryArn": repo.repository_arn().unwrap_or("-"),
                        "repositoryUri": repo.repository_uri().unwrap_or("-"),
                        "createdAt": repo.created_at().map(|d| d.to_string()).unwrap_or_else(|| "-".to_string()),
                    })
                })
                .collect();
            Ok(json!({ "repositories": repos }))
        }

        // =====================================================================
        // EFS Operations
        // =====================================================================
        ("efs", "describe_file_systems") => {
            let response = clients.efs.describe_file_systems().send().await?;
            let filesystems: Vec<Value> = response
                .file_systems()
                .iter()
                .map(|fs| {
                    json!({
                        "FileSystemId": fs.file_system_id(),
                        "Name": fs.name().unwrap_or("-"),
                        "LifeCycleState": fs.life_cycle_state().as_str(),
                        "SizeInBytes": fs.size_in_bytes().map(|s| s.value()).unwrap_or(0),
                        "PerformanceMode": fs.performance_mode().as_str(),
                    })
                })
                .collect();
            Ok(json!({ "file_systems": filesystems }))
        }

        // =====================================================================
        // EMR Operations
        // =====================================================================
        ("emr", "list_clusters") => {
            let response = clients.emr.list_clusters().send().await?;
            let clusters: Vec<Value> = response
                .clusters()
                .iter()
                .map(|cluster| {
                    json!({
                        "Id": cluster.id().unwrap_or("-"),
                        "Name": cluster.name().unwrap_or("-"),
                        "State": cluster.status().and_then(|s| s.state()).map(|s| s.as_str()).unwrap_or("-"),
                        "NormalizedInstanceHours": cluster.normalized_instance_hours().unwrap_or(0),
                    })
                })
                .collect();
            Ok(json!({ "clusters": clusters }))
        }

        // =====================================================================
        // EventBridge Operations
        // =====================================================================
        ("eventbridge", "list_rules") => {
            let response = clients.eventbridge.list_rules().send().await?;
            let rules: Vec<Value> = response
                .rules()
                .iter()
                .map(|rule| {
                    json!({
                        "Name": rule.name().unwrap_or("-"),
                        "Arn": rule.arn().unwrap_or("-"),
                        "State": rule.state().map(|s| s.as_str()).unwrap_or("-"),
                        "EventBusName": rule.event_bus_name().unwrap_or("-"),
                        "Description": rule.description().unwrap_or("-"),
                    })
                })
                .collect();
            Ok(json!({ "rules": rules }))
        }

        ("eventbridge", "list_event_buses") => {
            let response = clients.eventbridge.list_event_buses().send().await?;
            let buses: Vec<Value> = response
                .event_buses()
                .iter()
                .map(|bus| {
                    json!({
                        "Name": bus.name().unwrap_or("-"),
                        "Arn": bus.arn().unwrap_or("-"),
                    })
                })
                .collect();
            Ok(json!({ "event_buses": buses }))
        }

        // =====================================================================
        // Firehose Operations
        // =====================================================================
        ("firehose", "list_delivery_streams") => {
            let response = clients.firehose.list_delivery_streams().send().await?;
            let streams: Vec<Value> = response
                .delivery_stream_names()
                .iter()
                .map(|name| {
                    json!({
                        "DeliveryStreamName": name,
                    })
                })
                .collect();
            Ok(json!({ "delivery_streams": streams }))
        }

        // =====================================================================
        // FSx Operations
        // =====================================================================
        ("fsx", "describe_file_systems") => {
            let response = clients.fsx.describe_file_systems().send().await?;
            let filesystems: Vec<Value> = response
                .file_systems()
                .iter()
                .map(|fs| {
                    json!({
                        "FileSystemId": fs.file_system_id().unwrap_or("-"),
                        "FileSystemType": fs.file_system_type().map(|t| t.as_str()).unwrap_or("-"),
                        "Lifecycle": fs.lifecycle().map(|l| l.as_str()).unwrap_or("-"),
                        "StorageCapacity": fs.storage_capacity().unwrap_or(0),
                        "DNSName": fs.dns_name().unwrap_or("-"),
                    })
                })
                .collect();
            Ok(json!({ "file_systems": filesystems }))
        }

        // =====================================================================
        // Glue Operations
        // =====================================================================
        ("glue", "get_databases") => {
            let response = clients.glue.get_databases().send().await?;
            let databases: Vec<Value> = response
                .database_list()
                .iter()
                .map(|db| {
                    json!({
                        "Name": db.name(),
                        "Description": db.description().unwrap_or("-"),
                        "LocationUri": db.location_uri().unwrap_or("-"),
                    })
                })
                .collect();
            Ok(json!({ "databases": databases }))
        }

        ("glue", "get_jobs") => {
            let response = clients.glue.get_jobs().send().await?;
            let jobs: Vec<Value> = response
                .jobs()
                .iter()
                .map(|job| {
                    json!({
                        "Name": job.name().unwrap_or("-"),
                        "Role": job.role().unwrap_or("-"),
                        "CreatedOn": job.created_on().map(|d| d.to_string()).unwrap_or_else(|| "-".to_string()),
                        "LastModifiedOn": job.last_modified_on().map(|d| d.to_string()).unwrap_or_else(|| "-".to_string()),
                    })
                })
                .collect();
            Ok(json!({ "jobs": jobs }))
        }

        ("glue", "get_crawlers") => {
            let response = clients.glue.get_crawlers().send().await?;
            let crawlers: Vec<Value> = response
                .crawlers()
                .iter()
                .map(|crawler| {
                    json!({
                        "Name": crawler.name().unwrap_or("-"),
                        "State": crawler.state().map(|s| s.as_str()).unwrap_or("-"),
                        "DatabaseName": crawler.database_name().unwrap_or("-"),
                    })
                })
                .collect();
            Ok(json!({ "crawlers": crawlers }))
        }

        // =====================================================================
        // GuardDuty Operations
        // =====================================================================
        ("guardduty", "list_detectors") => {
            let response = clients.guardduty.list_detectors().send().await?;
            let detectors: Vec<Value> = response
                .detector_ids()
                .iter()
                .map(|id| {
                    json!({
                        "DetectorId": id,
                    })
                })
                .collect();
            Ok(json!({ "detectors": detectors }))
        }

        // =====================================================================
        // Inspector2 Operations
        // =====================================================================
        ("inspector2", "list_findings") => {
            let response = clients.inspector2.list_findings().send().await?;
            let findings: Vec<Value> = response
                .findings()
                .iter()
                .map(|finding| {
                    json!({
                        "findingArn": finding.finding_arn(),
                        "title": finding.title().unwrap_or("-"),
                        "severity": finding.severity().as_str(),
                        "status": finding.status().as_str(),
                        "type": finding.r#type().as_str(),
                    })
                })
                .collect();
            Ok(json!({ "findings": findings }))
        }

        // =====================================================================
        // Kinesis Operations
        // =====================================================================
        ("kinesis", "list_streams_with_details") => {
            let response = clients.kinesis.list_streams().send().await?;
            let mut streams: Vec<Value> = Vec::new();
            
            for summary in response.stream_summaries() {
                streams.push(json!({
                    "StreamName": summary.stream_name(),
                    "StreamARN": summary.stream_arn(),
                    "StreamStatus": summary.stream_status().as_str(),
                    "StreamModeDetails": summary.stream_mode_details().map(|m| m.stream_mode().as_str()).unwrap_or("-"),
                }));
            }
            Ok(json!({ "streams": streams }))
        }

        // =====================================================================
        // KMS Operations
        // =====================================================================
        ("kms", "list_keys_with_details") => {
            let response = clients.kms.list_keys().send().await?;
            let mut keys: Vec<Value> = Vec::new();
            
            for key in response.keys() {
                let key_id = key.key_id().unwrap_or("-");
                if let Ok(desc_response) = clients.kms.describe_key().key_id(key_id).send().await {
                    if let Some(metadata) = desc_response.key_metadata() {
                        keys.push(json!({
                            "KeyId": metadata.key_id(),
                            "KeyArn": metadata.arn().unwrap_or("-"),
                            "KeyState": metadata.key_state().map(|s| s.as_str()).unwrap_or("-"),
                            "KeyUsage": metadata.key_usage().map(|u| u.as_str()).unwrap_or("-"),
                            "KeySpec": metadata.key_spec().map(|s| s.as_str()).unwrap_or("-"),
                        }));
                    }
                }
            }
            Ok(json!({ "keys": keys }))
        }

        // =====================================================================
        // Lightsail Operations
        // =====================================================================
        ("lightsail", "get_instances") => {
            let response = clients.lightsail.get_instances().send().await?;
            let instances: Vec<Value> = response
                .instances()
                .iter()
                .map(|instance| {
                    json!({
                        "name": instance.name().unwrap_or("-"),
                        "arn": instance.arn().unwrap_or("-"),
                        "state": instance.state().and_then(|s| s.name()).unwrap_or("-"),
                        "blueprintName": instance.blueprint_name().unwrap_or("-"),
                        "bundleId": instance.bundle_id().unwrap_or("-"),
                        "publicIpAddress": instance.public_ip_address().unwrap_or("-"),
                    })
                })
                .collect();
            Ok(json!({ "instances": instances }))
        }

        // =====================================================================
        // MediaConvert Operations
        // =====================================================================
        ("mediaconvert", "list_queues") => {
            let response = clients.mediaconvert.list_queues().send().await?;
            let queues: Vec<Value> = response
                .queues()
                .iter()
                .map(|queue| {
                    json!({
                        "Name": queue.name(),
                        "Arn": queue.arn().unwrap_or("-"),
                        "Status": queue.status().map(|s| s.as_str()).unwrap_or("-"),
                        "Type": queue.r#type().map(|t| t.as_str()).unwrap_or("-"),
                        "PricingPlan": queue.pricing_plan().map(|p| p.as_str()).unwrap_or("-"),
                    })
                })
                .collect();
            Ok(json!({ "queues": queues }))
        }

        // =====================================================================
        // MemoryDB Operations
        // =====================================================================
        ("memorydb", "describe_clusters") => {
            let response = clients.memorydb.describe_clusters().send().await?;
            let clusters: Vec<Value> = response
                .clusters()
                .iter()
                .map(|cluster| {
                    json!({
                        "Name": cluster.name().unwrap_or("-"),
                        "ARN": cluster.arn().unwrap_or("-"),
                        "Status": cluster.status().unwrap_or("-"),
                        "NodeType": cluster.node_type().unwrap_or("-"),
                        "NumberOfShards": cluster.number_of_shards().unwrap_or(0),
                        "Engine": cluster.engine().unwrap_or("-"),
                    })
                })
                .collect();
            Ok(json!({ "clusters": clusters }))
        }

        // =====================================================================
        // MQ Operations
        // =====================================================================
        ("mq", "list_brokers") => {
            let response = clients.mq.list_brokers().send().await?;
            let brokers: Vec<Value> = response
                .broker_summaries()
                .iter()
                .map(|broker| {
                    json!({
                        "BrokerId": broker.broker_id().unwrap_or("-"),
                        "BrokerName": broker.broker_name().unwrap_or("-"),
                        "BrokerState": broker.broker_state().map(|s| s.as_str()).unwrap_or("-"),
                        "EngineType": broker.engine_type().map(|e| e.as_str()).unwrap_or("-"),
                        "DeploymentMode": broker.deployment_mode().map(|d| d.as_str()).unwrap_or("-"),
                    })
                })
                .collect();
            Ok(json!({ "brokers": brokers }))
        }

        // =====================================================================
        // Neptune Operations
        // =====================================================================
        ("neptune", "describe_db_clusters") => {
            let response = clients.neptune.describe_db_clusters().send().await?;
            let clusters: Vec<Value> = response
                .db_clusters()
                .iter()
                .map(|cluster| {
                    json!({
                        "DBClusterIdentifier": cluster.db_cluster_identifier().unwrap_or("-"),
                        "Status": cluster.status().unwrap_or("-"),
                        "Engine": cluster.engine().unwrap_or("-"),
                        "Endpoint": cluster.endpoint().unwrap_or("-"),
                    })
                })
                .collect();
            Ok(json!({ "db_clusters": clusters }))
        }

        // =====================================================================
        // OpenSearch Operations
        // =====================================================================
        ("opensearch", "list_domain_names") => {
            let response = clients.opensearch.list_domain_names().send().await?;
            let domains: Vec<Value> = response
                .domain_names()
                .iter()
                .map(|domain| {
                    json!({
                        "DomainName": domain.domain_name().unwrap_or("-"),
                        "EngineType": domain.engine_type().map(|e| e.as_str()).unwrap_or("-"),
                    })
                })
                .collect();
            Ok(json!({ "domain_names": domains }))
        }

        // =====================================================================
        // Organizations Operations
        // =====================================================================
        ("organizations", "list_accounts") => {
            let response = clients.organizations.list_accounts().send().await?;
            let accounts: Vec<Value> = response
                .accounts()
                .iter()
                .map(|account| {
                    json!({
                        "Id": account.id().unwrap_or("-"),
                        "Name": account.name().unwrap_or("-"),
                        "Email": account.email().unwrap_or("-"),
                        "Status": account.status().map(|s| s.as_str()).unwrap_or("-"),
                        "JoinedTimestamp": account.joined_timestamp().map(|d| d.to_string()).unwrap_or_else(|| "-".to_string()),
                    })
                })
                .collect();
            Ok(json!({ "accounts": accounts }))
        }

        // =====================================================================
        // Redshift Operations
        // =====================================================================
        ("redshift", "describe_clusters") => {
            let response = clients.redshift.describe_clusters().send().await?;
            let clusters: Vec<Value> = response
                .clusters()
                .iter()
                .map(|cluster| {
                    json!({
                        "ClusterIdentifier": cluster.cluster_identifier().unwrap_or("-"),
                        "ClusterStatus": cluster.cluster_status().unwrap_or("-"),
                        "NodeType": cluster.node_type().unwrap_or("-"),
                        "NumberOfNodes": cluster.number_of_nodes().unwrap_or(0),
                        "DBName": cluster.db_name().unwrap_or("-"),
                        "Endpoint": cluster.endpoint().and_then(|e| e.address()).unwrap_or("-"),
                    })
                })
                .collect();
            Ok(json!({ "clusters": clusters }))
        }

        // =====================================================================
        // SageMaker Operations
        // =====================================================================
        ("sagemaker", "list_notebook_instances") => {
            let response = clients.sagemaker.list_notebook_instances().send().await?;
            let notebooks: Vec<Value> = response
                .notebook_instances()
                .iter()
                .map(|nb| {
                    json!({
                        "NotebookInstanceName": nb.notebook_instance_name().unwrap_or("-"),
                        "NotebookInstanceArn": nb.notebook_instance_arn().unwrap_or("-"),
                        "NotebookInstanceStatus": nb.notebook_instance_status().map(|s| s.as_str()).unwrap_or("-"),
                        "InstanceType": nb.instance_type().map(|t| t.as_str()).unwrap_or("-"),
                        "CreationTime": nb.creation_time().map(|d| d.to_string()).unwrap_or_else(|| "-".to_string()),
                    })
                })
                .collect();
            Ok(json!({ "notebook_instances": notebooks }))
        }

        ("sagemaker", "list_endpoints") => {
            let response = clients.sagemaker.list_endpoints().send().await?;
            let endpoints: Vec<Value> = response
                .endpoints()
                .iter()
                .map(|ep| {
                    json!({
                        "EndpointName": ep.endpoint_name(),
                        "EndpointArn": ep.endpoint_arn(),
                        "EndpointStatus": ep.endpoint_status().map(|s| s.as_str()).unwrap_or("-"),
                        "CreationTime": ep.creation_time().map(|d| d.to_string()).unwrap_or_else(|| "-".to_string()),
                        "LastModifiedTime": ep.last_modified_time().map(|d| d.to_string()).unwrap_or_else(|| "-".to_string()),
                    })
                })
                .collect();
            Ok(json!({ "endpoints": endpoints }))
        }

        // =====================================================================
        // SES v2 Operations
        // =====================================================================
        ("sesv2", "list_email_identities") => {
            let response = clients.sesv2.list_email_identities().send().await?;
            let identities: Vec<Value> = response
                .email_identities()
                .iter()
                .map(|identity| {
                    json!({
                        "IdentityName": identity.identity_name().unwrap_or("-"),
                        "IdentityType": identity.identity_type().map(|t| t.as_str()).unwrap_or("-"),
                        "SendingEnabled": if identity.sending_enabled() { "Yes" } else { "No" },
                    })
                })
                .collect();
            Ok(json!({ "email_identities": identities }))
        }

        // =====================================================================
        // Shield Operations
        // =====================================================================
        ("shield", "list_protections") => {
            let response = clients.shield.list_protections().send().await?;
            let protections: Vec<Value> = response
                .protections()
                .iter()
                .map(|protection| {
                    json!({
                        "Id": protection.id().unwrap_or("-"),
                        "Name": protection.name().unwrap_or("-"),
                        "ResourceArn": protection.resource_arn().unwrap_or("-"),
                    })
                })
                .collect();
            Ok(json!({ "protections": protections }))
        }

        // =====================================================================
        // Step Functions Operations
        // =====================================================================
        ("stepfunctions", "list_state_machines") => {
            let response = clients.sfn.list_state_machines().send().await?;
            let machines: Vec<Value> = response
                .state_machines()
                .iter()
                .map(|sm| {
                    json!({
                        "name": sm.name(),
                        "stateMachineArn": sm.state_machine_arn(),
                        "type": sm.r#type().as_str(),
                        "creationDate": sm.creation_date().to_string(),
                    })
                })
                .collect();
            Ok(json!({ "state_machines": machines }))
        }

        // =====================================================================
        // Storage Gateway Operations
        // =====================================================================
        ("storagegateway", "list_gateways") => {
            let response = clients.storagegateway.list_gateways().send().await?;
            let gateways: Vec<Value> = response
                .gateways()
                .iter()
                .map(|gw| {
                    json!({
                        "GatewayId": gw.gateway_id().unwrap_or("-"),
                        "GatewayName": gw.gateway_name().unwrap_or("-"),
                        "GatewayType": gw.gateway_type().unwrap_or("-"),
                        "GatewayOperationalState": gw.gateway_operational_state().unwrap_or("-"),
                    })
                })
                .collect();
            Ok(json!({ "gateways": gateways }))
        }

        // =====================================================================
        // STS Operations
        // =====================================================================
        ("sts", "get_caller_identity") => {
            let response = clients.sts.get_caller_identity().send().await?;
            let identity = json!({
                "Account": response.account().unwrap_or("-"),
                "UserId": response.user_id().unwrap_or("-"),
                "Arn": response.arn().unwrap_or("-"),
            });
            Ok(json!({ "identity": [identity] }))
        }

        // =====================================================================
        // Transfer Operations
        // =====================================================================
        ("transfer", "list_servers") => {
            let response = clients.transfer.list_servers().send().await?;
            let servers: Vec<Value> = response
                .servers()
                .iter()
                .map(|server| {
                    json!({
                        "ServerId": server.server_id().unwrap_or("-"),
                        "State": server.state().map(|s| s.as_str()).unwrap_or("-"),
                        "EndpointType": server.endpoint_type().map(|e| e.as_str()).unwrap_or("-"),
                        "Domain": server.domain().map(|d| d.as_str()).unwrap_or("-"),
                        "UserCount": server.user_count().unwrap_or(0),
                    })
                })
                .collect();
            Ok(json!({ "servers": servers }))
        }

        // =====================================================================
        // WAFv2 Operations
        // =====================================================================
        ("wafv2", "list_web_acls") => {
            let response = clients.wafv2.list_web_acls()
                .scope(aws_sdk_wafv2::types::Scope::Regional)
                .send()
                .await?;
            let acls: Vec<Value> = response
                .web_acls()
                .iter()
                .map(|acl| {
                    json!({
                        "Id": acl.id().unwrap_or("-"),
                        "Name": acl.name().unwrap_or("-"),
                        "Description": acl.description().unwrap_or("-"),
                    })
                })
                .collect();
            Ok(json!({ "web_acls": acls }))
        }

        // =====================================================================
        // WorkSpaces Operations
        // =====================================================================
        ("workspaces", "describe_workspaces") => {
            let response = clients.workspaces.describe_workspaces().send().await?;
            let workspaces: Vec<Value> = response
                .workspaces()
                .iter()
                .map(|ws| {
                    json!({
                        "WorkspaceId": ws.workspace_id().unwrap_or("-"),
                        "UserName": ws.user_name().unwrap_or("-"),
                        "State": ws.state().map(|s| s.as_str()).unwrap_or("-"),
                        "BundleId": ws.bundle_id().unwrap_or("-"),
                        "DirectoryId": ws.directory_id().unwrap_or("-"),
                    })
                })
                .collect();
            Ok(json!({ "workspaces": workspaces }))
        }

        // =====================================================================
        // X-Ray Operations
        // =====================================================================
        ("xray", "get_groups") => {
            let response = clients.xray.get_groups().send().await?;
            let groups: Vec<Value> = response
                .groups()
                .iter()
                .map(|group| {
                    json!({
                        "GroupName": group.group_name().unwrap_or("-"),
                        "GroupARN": group.group_arn().unwrap_or("-"),
                        "FilterExpression": group.filter_expression().unwrap_or("-"),
                    })
                })
                .collect();
            Ok(json!({ "groups": groups }))
        }

        // =====================================================================
        // App Runner Operations
        // =====================================================================
        ("apprunner", "list_services") => {
            let response = clients.apprunner.list_services().send().await?;
            let services: Vec<Value> = response
                .service_summary_list()
                .iter()
                .map(|service| {
                    json!({
                        "ServiceName": service.service_name(),
                        "ServiceArn": service.service_arn(),
                        "ServiceUrl": service.service_url().unwrap_or("-"),
                        "Status": service.status().map(|s| s.as_str()).unwrap_or("-"),
                        "CreatedAt": service.created_at().map(|d| d.to_string()).unwrap_or_else(|| "-".to_string()),
                    })
                })
                .collect();
            Ok(json!({ "services": services }))
        }

        // =====================================================================
        // AppSync Operations
        // =====================================================================
        ("appsync", "list_graphql_apis") => {
            let response = clients.appsync.list_graphql_apis().send().await?;
            let apis: Vec<Value> = response
                .graphql_apis()
                .iter()
                .map(|api| {
                    json!({
                        "apiId": api.api_id().unwrap_or("-"),
                        "name": api.name().unwrap_or("-"),
                        "authenticationType": api.authentication_type().map(|a| a.as_str()).unwrap_or("-"),
                    })
                })
                .collect();
            Ok(json!({ "graphql_apis": apis }))
        }

        // =====================================================================
        // Amplify Operations
        // =====================================================================
        ("amplify", "list_apps") => {
            let response = clients.amplify.list_apps().send().await?;
            let apps: Vec<Value> = response
                .apps()
                .iter()
                .map(|app| {
                    json!({
                        "appId": app.app_id(),
                        "name": app.name(),
                        "platform": app.platform().as_str(),
                        "repository": app.repository(),
                        "createTime": app.create_time().to_string(),
                    })
                })
                .collect();
            Ok(json!({ "apps": apps }))
        }

        // =====================================================================
        // Bedrock Operations
        // =====================================================================
        ("bedrock", "list_foundation_models") => {
            let response = clients.bedrock.list_foundation_models().send().await?;
            let models: Vec<Value> = response
                .model_summaries()
                .iter()
                .map(|model| {
                    let input_modalities: Vec<String> = model.input_modalities()
                        .iter()
                        .map(|m| m.as_str().to_string())
                        .collect();
                    json!({
                        "modelId": model.model_id(),
                        "modelName": model.model_name().unwrap_or("-"),
                        "providerName": model.provider_name().unwrap_or("-"),
                        "inputModalities": input_modalities.join(", "),
                    })
                })
                .collect();
            Ok(json!({ "models": models }))
        }

        // =====================================================================
        // QuickSight Operations
        // =====================================================================
        ("quicksight", "list_dashboards") => {
            // Get account ID from STS
            let sts_response = clients.sts.get_caller_identity().send().await?;
            let account_id = sts_response.account().unwrap_or("-");
            
            let response = clients.quicksight.list_dashboards()
                .aws_account_id(account_id)
                .send()
                .await?;
            let dashboards: Vec<Value> = response
                .dashboard_summary_list()
                .iter()
                .map(|dashboard| {
                    json!({
                        "DashboardId": dashboard.dashboard_id().unwrap_or("-"),
                        "Name": dashboard.name().unwrap_or("-"),
                        "CreatedTime": dashboard.created_time().map(|d| d.to_string()).unwrap_or_else(|| "-".to_string()),
                        "PublishedVersionNumber": dashboard.published_version_number().unwrap_or(0),
                    })
                })
                .collect();
            Ok(json!({ "dashboards": dashboards }))
        }

        // =====================================================================
        // DataSync Operations
        // =====================================================================
        ("datasync", "list_tasks") => {
            let response = clients.datasync.list_tasks().send().await?;
            let tasks: Vec<Value> = response
                .tasks()
                .iter()
                .map(|task| {
                    json!({
                        "TaskArn": task.task_arn().unwrap_or("-"),
                        "Name": task.name().unwrap_or("-"),
                        "Status": task.status().map(|s| s.as_str()).unwrap_or("-"),
                    })
                })
                .collect();
            Ok(json!({ "tasks": tasks }))
        }

        // =====================================================================
        // DMS Operations
        // =====================================================================
        ("dms", "describe_replication_instances") => {
            let response = clients.dms.describe_replication_instances().send().await?;
            let instances: Vec<Value> = response
                .replication_instances()
                .iter()
                .map(|instance| {
                    json!({
                        "ReplicationInstanceIdentifier": instance.replication_instance_identifier().unwrap_or("-"),
                        "ReplicationInstanceStatus": instance.replication_instance_status().unwrap_or("-"),
                        "ReplicationInstanceClass": instance.replication_instance_class().unwrap_or("-"),
                        "EngineVersion": instance.engine_version().unwrap_or("-"),
                        "AvailabilityZone": instance.availability_zone().unwrap_or("-"),
                    })
                })
                .collect();
            Ok(json!({ "replication_instances": instances }))
        }

        // =====================================================================
        // Elastic Beanstalk Operations
        // =====================================================================
        ("elasticbeanstalk", "describe_applications") => {
            let response = clients.elasticbeanstalk.describe_applications().send().await?;
            let applications: Vec<Value> = response
                .applications()
                .iter()
                .map(|app| {
                    json!({
                        "ApplicationName": app.application_name().unwrap_or("-"),
                        "ApplicationArn": app.application_arn().unwrap_or("-"),
                        "Description": app.description().unwrap_or("-"),
                        "DateCreated": app.date_created().map(|d| d.to_string()).unwrap_or_else(|| "-".to_string()),
                        "DateUpdated": app.date_updated().map(|d| d.to_string()).unwrap_or_else(|| "-".to_string()),
                    })
                })
                .collect();
            Ok(json!({ "applications": applications }))
        }

        ("elasticbeanstalk", "describe_environments") => {
            let response = clients.elasticbeanstalk.describe_environments().send().await?;
            let environments: Vec<Value> = response
                .environments()
                .iter()
                .map(|env| {
                    json!({
                        "EnvironmentName": env.environment_name().unwrap_or("-"),
                        "EnvironmentId": env.environment_id().unwrap_or("-"),
                        "Status": env.status().map(|s| s.as_str()).unwrap_or("-"),
                        "Health": env.health().map(|h| h.as_str()).unwrap_or("-"),
                        "ApplicationName": env.application_name().unwrap_or("-"),
                        "CNAME": env.cname().unwrap_or("-"),
                    })
                })
                .collect();
            Ok(json!({ "environments": environments }))
        }

        // =====================================================================
        // Unknown operation
        // =====================================================================
        _ => Err(anyhow!(
            "Unknown SDK operation: service='{}', method='{}'",
            service,
            method
        )),
    }
}