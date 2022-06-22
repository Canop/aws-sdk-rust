// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn serialize_structure_crate_input_batch_execute_statement_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::BatchExecuteStatementInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_1) = &input.sqls {
        let mut array_2 = object.key("Sqls").start_array();
        for item_3 in var_1 {
            {
                array_2.value().string(item_3.as_str());
            }
        }
        array_2.finish();
    }
    if let Some(var_4) = &input.cluster_identifier {
        object.key("ClusterIdentifier").string(var_4.as_str());
    }
    if let Some(var_5) = &input.secret_arn {
        object.key("SecretArn").string(var_5.as_str());
    }
    if let Some(var_6) = &input.db_user {
        object.key("DbUser").string(var_6.as_str());
    }
    if let Some(var_7) = &input.database {
        object.key("Database").string(var_7.as_str());
    }
    if let Some(var_8) = &input.with_event {
        object.key("WithEvent").boolean(*var_8);
    }
    if let Some(var_9) = &input.statement_name {
        object.key("StatementName").string(var_9.as_str());
    }
    if let Some(var_10) = &input.workgroup_name {
        object.key("WorkgroupName").string(var_10.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_cancel_statement_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::CancelStatementInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_11) = &input.id {
        object.key("Id").string(var_11.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_describe_statement_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::DescribeStatementInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_12) = &input.id {
        object.key("Id").string(var_12.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_describe_table_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::DescribeTableInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_13) = &input.cluster_identifier {
        object.key("ClusterIdentifier").string(var_13.as_str());
    }
    if let Some(var_14) = &input.secret_arn {
        object.key("SecretArn").string(var_14.as_str());
    }
    if let Some(var_15) = &input.db_user {
        object.key("DbUser").string(var_15.as_str());
    }
    if let Some(var_16) = &input.database {
        object.key("Database").string(var_16.as_str());
    }
    if let Some(var_17) = &input.connected_database {
        object.key("ConnectedDatabase").string(var_17.as_str());
    }
    if let Some(var_18) = &input.schema {
        object.key("Schema").string(var_18.as_str());
    }
    if let Some(var_19) = &input.table {
        object.key("Table").string(var_19.as_str());
    }
    if let Some(var_20) = &input.next_token {
        object.key("NextToken").string(var_20.as_str());
    }
    if input.max_results != 0 {
        object.key("MaxResults").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((input.max_results).into()),
        );
    }
    if let Some(var_21) = &input.workgroup_name {
        object.key("WorkgroupName").string(var_21.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_execute_statement_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::ExecuteStatementInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_22) = &input.sql {
        object.key("Sql").string(var_22.as_str());
    }
    if let Some(var_23) = &input.cluster_identifier {
        object.key("ClusterIdentifier").string(var_23.as_str());
    }
    if let Some(var_24) = &input.secret_arn {
        object.key("SecretArn").string(var_24.as_str());
    }
    if let Some(var_25) = &input.db_user {
        object.key("DbUser").string(var_25.as_str());
    }
    if let Some(var_26) = &input.database {
        object.key("Database").string(var_26.as_str());
    }
    if let Some(var_27) = &input.with_event {
        object.key("WithEvent").boolean(*var_27);
    }
    if let Some(var_28) = &input.statement_name {
        object.key("StatementName").string(var_28.as_str());
    }
    if let Some(var_29) = &input.parameters {
        let mut array_30 = object.key("Parameters").start_array();
        for item_31 in var_29 {
            {
                let mut object_32 = array_30.value().start_object();
                crate::json_ser::serialize_structure_crate_model_sql_parameter(
                    &mut object_32,
                    item_31,
                )?;
                object_32.finish();
            }
        }
        array_30.finish();
    }
    if let Some(var_33) = &input.workgroup_name {
        object.key("WorkgroupName").string(var_33.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_get_statement_result_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::GetStatementResultInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_34) = &input.id {
        object.key("Id").string(var_34.as_str());
    }
    if let Some(var_35) = &input.next_token {
        object.key("NextToken").string(var_35.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_list_databases_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::ListDatabasesInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_36) = &input.cluster_identifier {
        object.key("ClusterIdentifier").string(var_36.as_str());
    }
    if let Some(var_37) = &input.database {
        object.key("Database").string(var_37.as_str());
    }
    if let Some(var_38) = &input.secret_arn {
        object.key("SecretArn").string(var_38.as_str());
    }
    if let Some(var_39) = &input.db_user {
        object.key("DbUser").string(var_39.as_str());
    }
    if let Some(var_40) = &input.next_token {
        object.key("NextToken").string(var_40.as_str());
    }
    if input.max_results != 0 {
        object.key("MaxResults").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((input.max_results).into()),
        );
    }
    if let Some(var_41) = &input.workgroup_name {
        object.key("WorkgroupName").string(var_41.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_list_schemas_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::ListSchemasInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_42) = &input.cluster_identifier {
        object.key("ClusterIdentifier").string(var_42.as_str());
    }
    if let Some(var_43) = &input.secret_arn {
        object.key("SecretArn").string(var_43.as_str());
    }
    if let Some(var_44) = &input.db_user {
        object.key("DbUser").string(var_44.as_str());
    }
    if let Some(var_45) = &input.database {
        object.key("Database").string(var_45.as_str());
    }
    if let Some(var_46) = &input.connected_database {
        object.key("ConnectedDatabase").string(var_46.as_str());
    }
    if let Some(var_47) = &input.schema_pattern {
        object.key("SchemaPattern").string(var_47.as_str());
    }
    if let Some(var_48) = &input.next_token {
        object.key("NextToken").string(var_48.as_str());
    }
    if input.max_results != 0 {
        object.key("MaxResults").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((input.max_results).into()),
        );
    }
    if let Some(var_49) = &input.workgroup_name {
        object.key("WorkgroupName").string(var_49.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_list_statements_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::ListStatementsInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_50) = &input.next_token {
        object.key("NextToken").string(var_50.as_str());
    }
    if input.max_results != 0 {
        object.key("MaxResults").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((input.max_results).into()),
        );
    }
    if let Some(var_51) = &input.statement_name {
        object.key("StatementName").string(var_51.as_str());
    }
    if let Some(var_52) = &input.status {
        object.key("Status").string(var_52.as_str());
    }
    if let Some(var_53) = &input.role_level {
        object.key("RoleLevel").boolean(*var_53);
    }
    Ok(())
}

pub fn serialize_structure_crate_input_list_tables_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::ListTablesInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_54) = &input.cluster_identifier {
        object.key("ClusterIdentifier").string(var_54.as_str());
    }
    if let Some(var_55) = &input.secret_arn {
        object.key("SecretArn").string(var_55.as_str());
    }
    if let Some(var_56) = &input.db_user {
        object.key("DbUser").string(var_56.as_str());
    }
    if let Some(var_57) = &input.database {
        object.key("Database").string(var_57.as_str());
    }
    if let Some(var_58) = &input.connected_database {
        object.key("ConnectedDatabase").string(var_58.as_str());
    }
    if let Some(var_59) = &input.schema_pattern {
        object.key("SchemaPattern").string(var_59.as_str());
    }
    if let Some(var_60) = &input.table_pattern {
        object.key("TablePattern").string(var_60.as_str());
    }
    if let Some(var_61) = &input.next_token {
        object.key("NextToken").string(var_61.as_str());
    }
    if input.max_results != 0 {
        object.key("MaxResults").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((input.max_results).into()),
        );
    }
    if let Some(var_62) = &input.workgroup_name {
        object.key("WorkgroupName").string(var_62.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_model_sql_parameter(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::SqlParameter,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_63) = &input.name {
        object.key("name").string(var_63.as_str());
    }
    if let Some(var_64) = &input.value {
        object.key("value").string(var_64.as_str());
    }
    Ok(())
}
