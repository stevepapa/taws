use anyhow::Result;
use aws_config::BehaviorVersion;
use aws_sdk_ec2::Client as Ec2Client;
use aws_sdk_iam::Client as IamClient;
use aws_sdk_s3::Client as S3Client;
use aws_sdk_lambda::Client as LambdaClient;
use aws_sdk_rds::Client as RdsClient;
use aws_sdk_dynamodb::Client as DynamoDbClient;
use aws_sdk_ecs::Client as EcsClient;
use aws_sdk_cloudwatchlogs::Client as CloudWatchLogsClient;
use aws_sdk_sns::Client as SnsClient;
use aws_sdk_sqs::Client as SqsClient;
use aws_sdk_elasticloadbalancingv2::Client as ElbClient;
use aws_sdk_cloudformation::Client as CloudFormationClient;
use aws_sdk_secretsmanager::Client as SecretsManagerClient;
use aws_sdk_ssm::Client as SsmClient;
use aws_sdk_eks::Client as EksClient;
use aws_sdk_apigateway::Client as ApiGatewayClient;
use aws_sdk_route53::Client as Route53Client;
use aws_sdk_elasticache::Client as ElastiCacheClient;
// Batch 1
use aws_sdk_acm::Client as AcmClient;
use aws_sdk_athena::Client as AthenaClient;
use aws_sdk_autoscaling::Client as AutoScalingClient;
use aws_sdk_backup::Client as BackupClient;
use aws_sdk_batch::Client as BatchClient;
use aws_sdk_budgets::Client as BudgetsClient;
use aws_sdk_cloudfront::Client as CloudFrontClient;
use aws_sdk_cloudtrail::Client as CloudTrailClient;
use aws_sdk_codebuild::Client as CodeBuildClient;
use aws_sdk_codepipeline::Client as CodePipelineClient;
// Batch 2
use aws_sdk_cognitoidentityprovider::Client as CognitoIdpClient;
use aws_sdk_config::Client as ConfigClient;
use aws_sdk_directconnect::Client as DirectConnectClient;
use aws_sdk_ecr::Client as EcrClient;
use aws_sdk_efs::Client as EfsClient;
use aws_sdk_emr::Client as EmrClient;
use aws_sdk_eventbridge::Client as EventBridgeClient;
use aws_sdk_firehose::Client as FirehoseClient;
use aws_sdk_fsx::Client as FsxClient;
use aws_sdk_glue::Client as GlueClient;
// Batch 3
use aws_sdk_guardduty::Client as GuardDutyClient;
use aws_sdk_inspector2::Client as Inspector2Client;
use aws_sdk_kinesis::Client as KinesisClient;
use aws_sdk_kms::Client as KmsClient;
use aws_sdk_lightsail::Client as LightsailClient;
use aws_sdk_mediaconvert::Client as MediaConvertClient;
use aws_sdk_memorydb::Client as MemoryDbClient;
use aws_sdk_mq::Client as MqClient;
use aws_sdk_neptune::Client as NeptuneClient;
use aws_sdk_opensearch::Client as OpenSearchClient;
// Batch 4
use aws_sdk_organizations::Client as OrganizationsClient;
use aws_sdk_redshift::Client as RedshiftClient;
use aws_sdk_sagemaker::Client as SageMakerClient;
use aws_sdk_sesv2::Client as SesV2Client;
use aws_sdk_shield::Client as ShieldClient;
use aws_sdk_sfn::Client as SfnClient;
use aws_sdk_storagegateway::Client as StorageGatewayClient;
use aws_sdk_sts::Client as StsClient;
use aws_sdk_transfer::Client as TransferClient;
use aws_sdk_wafv2::Client as Wafv2Client;
// Batch 5
use aws_sdk_workspaces::Client as WorkSpacesClient;
use aws_sdk_xray::Client as XRayClient;
use aws_sdk_apprunner::Client as AppRunnerClient;
use aws_sdk_appsync::Client as AppSyncClient;
use aws_sdk_amplify::Client as AmplifyClient;
use aws_sdk_bedrock::Client as BedrockClient;
use aws_sdk_quicksight::Client as QuickSightClient;
use aws_sdk_datasync::Client as DataSyncClient;
use aws_sdk_databasemigration::Client as DmsClient;
use aws_sdk_elasticbeanstalk::Client as ElasticBeanstalkClient;

/// Container for all AWS service clients
pub struct AwsClients {
    pub ec2: Ec2Client,
    pub iam: IamClient,
    #[allow(dead_code)]
    pub s3: S3Client,
    #[allow(dead_code)]
    pub lambda: LambdaClient,
    #[allow(dead_code)]
    pub rds: RdsClient,
    #[allow(dead_code)]
    pub dynamodb: DynamoDbClient,
    #[allow(dead_code)]
    pub ecs: EcsClient,
    #[allow(dead_code)]
    pub logs: CloudWatchLogsClient,
    #[allow(dead_code)]
    pub sns: SnsClient,
    #[allow(dead_code)]
    pub sqs: SqsClient,
    #[allow(dead_code)]
    pub elb: ElbClient,
    #[allow(dead_code)]
    pub cloudformation: CloudFormationClient,
    #[allow(dead_code)]
    pub secretsmanager: SecretsManagerClient,
    #[allow(dead_code)]
    pub ssm: SsmClient,
    #[allow(dead_code)]
    pub eks: EksClient,
    #[allow(dead_code)]
    pub apigateway: ApiGatewayClient,
    #[allow(dead_code)]
    pub route53: Route53Client,
    #[allow(dead_code)]
    pub elasticache: ElastiCacheClient,
    // Batch 1
    #[allow(dead_code)]
    pub acm: AcmClient,
    #[allow(dead_code)]
    pub athena: AthenaClient,
    #[allow(dead_code)]
    pub autoscaling: AutoScalingClient,
    #[allow(dead_code)]
    pub backup: BackupClient,
    #[allow(dead_code)]
    pub batch: BatchClient,
    #[allow(dead_code)]
    pub budgets: BudgetsClient,
    #[allow(dead_code)]
    pub cloudfront: CloudFrontClient,
    #[allow(dead_code)]
    pub cloudtrail: CloudTrailClient,
    #[allow(dead_code)]
    pub codebuild: CodeBuildClient,
    #[allow(dead_code)]
    pub codepipeline: CodePipelineClient,
    // Batch 2
    #[allow(dead_code)]
    pub cognito_idp: CognitoIdpClient,
    #[allow(dead_code)]
    pub config: ConfigClient,
    #[allow(dead_code)]
    pub directconnect: DirectConnectClient,
    #[allow(dead_code)]
    pub ecr: EcrClient,
    #[allow(dead_code)]
    pub efs: EfsClient,
    #[allow(dead_code)]
    pub emr: EmrClient,
    #[allow(dead_code)]
    pub eventbridge: EventBridgeClient,
    #[allow(dead_code)]
    pub firehose: FirehoseClient,
    #[allow(dead_code)]
    pub fsx: FsxClient,
    #[allow(dead_code)]
    pub glue: GlueClient,
    // Batch 3
    #[allow(dead_code)]
    pub guardduty: GuardDutyClient,
    #[allow(dead_code)]
    pub inspector2: Inspector2Client,
    #[allow(dead_code)]
    pub kinesis: KinesisClient,
    #[allow(dead_code)]
    pub kms: KmsClient,
    #[allow(dead_code)]
    pub lightsail: LightsailClient,
    #[allow(dead_code)]
    pub mediaconvert: MediaConvertClient,
    #[allow(dead_code)]
    pub memorydb: MemoryDbClient,
    #[allow(dead_code)]
    pub mq: MqClient,
    #[allow(dead_code)]
    pub neptune: NeptuneClient,
    #[allow(dead_code)]
    pub opensearch: OpenSearchClient,
    // Batch 4
    #[allow(dead_code)]
    pub organizations: OrganizationsClient,
    #[allow(dead_code)]
    pub redshift: RedshiftClient,
    #[allow(dead_code)]
    pub sagemaker: SageMakerClient,
    #[allow(dead_code)]
    pub sesv2: SesV2Client,
    #[allow(dead_code)]
    pub shield: ShieldClient,
    #[allow(dead_code)]
    pub sfn: SfnClient,
    #[allow(dead_code)]
    pub storagegateway: StorageGatewayClient,
    #[allow(dead_code)]
    pub sts: StsClient,
    #[allow(dead_code)]
    pub transfer: TransferClient,
    #[allow(dead_code)]
    pub wafv2: Wafv2Client,
    // Batch 5
    #[allow(dead_code)]
    pub workspaces: WorkSpacesClient,
    #[allow(dead_code)]
    pub xray: XRayClient,
    #[allow(dead_code)]
    pub apprunner: AppRunnerClient,
    #[allow(dead_code)]
    pub appsync: AppSyncClient,
    #[allow(dead_code)]
    pub amplify: AmplifyClient,
    #[allow(dead_code)]
    pub bedrock: BedrockClient,
    #[allow(dead_code)]
    pub quicksight: QuickSightClient,
    #[allow(dead_code)]
    pub datasync: DataSyncClient,
    #[allow(dead_code)]
    pub dms: DmsClient,
    #[allow(dead_code)]
    pub elasticbeanstalk: ElasticBeanstalkClient,
}

impl AwsClients {
    /// Create all AWS clients for a given profile and region
    pub async fn new(profile: &str, region: &str) -> Result<(Self, String)> {
        let config = aws_config::defaults(BehaviorVersion::latest())
            .profile_name(profile)
            .region(aws_sdk_ec2::config::Region::new(region.to_string()))
            .load()
            .await;

        let actual_region = config
            .region()
            .map(|r| r.to_string())
            .unwrap_or_else(|| region.to_string());

        // IAM uses us-east-1 (global service)
        let iam_config = aws_config::defaults(BehaviorVersion::latest())
            .profile_name(profile)
            .region(aws_sdk_iam::config::Region::new("us-east-1".to_string()))
            .load()
            .await;

        // Global services config (us-east-1)
        let global_config = aws_config::defaults(BehaviorVersion::latest())
            .profile_name(profile)
            .region(aws_sdk_route53::config::Region::new("us-east-1".to_string()))
            .load()
            .await;

        let clients = Self {
            ec2: Ec2Client::new(&config),
            iam: IamClient::new(&iam_config),
            s3: S3Client::new(&config),
            lambda: LambdaClient::new(&config),
            rds: RdsClient::new(&config),
            dynamodb: DynamoDbClient::new(&config),
            ecs: EcsClient::new(&config),
            logs: CloudWatchLogsClient::new(&config),
            sns: SnsClient::new(&config),
            sqs: SqsClient::new(&config),
            elb: ElbClient::new(&config),
            cloudformation: CloudFormationClient::new(&config),
            secretsmanager: SecretsManagerClient::new(&config),
            ssm: SsmClient::new(&config),
            eks: EksClient::new(&config),
            apigateway: ApiGatewayClient::new(&config),
            route53: Route53Client::new(&global_config),
            elasticache: ElastiCacheClient::new(&config),
            // Batch 1
            acm: AcmClient::new(&config),
            athena: AthenaClient::new(&config),
            autoscaling: AutoScalingClient::new(&config),
            backup: BackupClient::new(&config),
            batch: BatchClient::new(&config),
            budgets: BudgetsClient::new(&global_config),
            cloudfront: CloudFrontClient::new(&global_config),
            cloudtrail: CloudTrailClient::new(&config),
            codebuild: CodeBuildClient::new(&config),
            codepipeline: CodePipelineClient::new(&config),
            // Batch 2
            cognito_idp: CognitoIdpClient::new(&config),
            config: ConfigClient::new(&config),
            directconnect: DirectConnectClient::new(&config),
            ecr: EcrClient::new(&config),
            efs: EfsClient::new(&config),
            emr: EmrClient::new(&config),
            eventbridge: EventBridgeClient::new(&config),
            firehose: FirehoseClient::new(&config),
            fsx: FsxClient::new(&config),
            glue: GlueClient::new(&config),
            // Batch 3
            guardduty: GuardDutyClient::new(&config),
            inspector2: Inspector2Client::new(&config),
            kinesis: KinesisClient::new(&config),
            kms: KmsClient::new(&config),
            lightsail: LightsailClient::new(&config),
            mediaconvert: MediaConvertClient::new(&config),
            memorydb: MemoryDbClient::new(&config),
            mq: MqClient::new(&config),
            neptune: NeptuneClient::new(&config),
            opensearch: OpenSearchClient::new(&config),
            // Batch 4
            organizations: OrganizationsClient::new(&global_config),
            redshift: RedshiftClient::new(&config),
            sagemaker: SageMakerClient::new(&config),
            sesv2: SesV2Client::new(&config),
            shield: ShieldClient::new(&global_config),
            sfn: SfnClient::new(&config),
            storagegateway: StorageGatewayClient::new(&config),
            sts: StsClient::new(&config),
            transfer: TransferClient::new(&config),
            wafv2: Wafv2Client::new(&config),
            // Batch 5
            workspaces: WorkSpacesClient::new(&config),
            xray: XRayClient::new(&config),
            apprunner: AppRunnerClient::new(&config),
            appsync: AppSyncClient::new(&config),
            amplify: AmplifyClient::new(&config),
            bedrock: BedrockClient::new(&config),
            quicksight: QuickSightClient::new(&config),
            datasync: DataSyncClient::new(&config),
            dms: DmsClient::new(&config),
            elasticbeanstalk: ElasticBeanstalkClient::new(&config),
        };

        Ok((clients, actual_region))
    }

    /// Recreate clients for a new region (keeps same profile)
    pub async fn switch_region(&mut self, profile: &str, region: &str) -> Result<String> {
        let (new_clients, actual_region) = Self::new(profile, region).await?;
        *self = new_clients;
        Ok(actual_region)
    }
}

/// Format AWS errors into user-friendly messages
pub fn format_aws_error(err: &anyhow::Error) -> String {
    let err_str = err.to_string();
    
    // Check for common AWS error patterns
    if err_str.contains("dispatch failure") {
        return "Connection failed - check internet/credentials".to_string();
    }
    if err_str.contains("InvalidClientTokenId") || err_str.contains("SignatureDoesNotMatch") {
        return "Invalid credentials - run 'aws configure'".to_string();
    }
    if err_str.contains("ExpiredToken") {
        return "Credentials expired - refresh or reconfigure".to_string();
    }
    if err_str.contains("AccessDenied") || err_str.contains("UnauthorizedAccess") {
        return "Access denied - check IAM permissions".to_string();
    }
    if err_str.contains("NoCredentialProviders") || err_str.contains("no credentials") {
        return "No credentials - run 'aws configure'".to_string();
    }
    if err_str.contains("timeout") || err_str.contains("Timeout") {
        return "Request timed out - check connection".to_string();
    }
    if err_str.contains("region") {
        return "Region error - check AWS_REGION".to_string();
    }
    
    // Default: truncate long errors
    if err_str.len() > 60 {
        format!("{}...", &err_str[..60])
    } else {
        err_str
    }
}
