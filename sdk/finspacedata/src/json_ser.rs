// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn serialize_structure_crate_input_associate_user_to_permission_group_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::AssociateUserToPermissionGroupInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_1) = &input.client_token {
        object.key("clientToken").string(var_1.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_create_changeset_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::CreateChangesetInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_2) = &input.change_type {
        object.key("changeType").string(var_2.as_str());
    }
    if let Some(var_3) = &input.client_token {
        object.key("clientToken").string(var_3.as_str());
    }
    if let Some(var_4) = &input.format_params {
        let mut object_5 = object.key("formatParams").start_object();
        for (key_6, value_7) in var_4 {
            {
                object_5.key(key_6).string(value_7.as_str());
            }
        }
        object_5.finish();
    }
    if let Some(var_8) = &input.source_params {
        let mut object_9 = object.key("sourceParams").start_object();
        for (key_10, value_11) in var_8 {
            {
                object_9.key(key_10).string(value_11.as_str());
            }
        }
        object_9.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_create_dataset_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::CreateDatasetInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_12) = &input.alias {
        object.key("alias").string(var_12.as_str());
    }
    if let Some(var_13) = &input.client_token {
        object.key("clientToken").string(var_13.as_str());
    }
    if let Some(var_14) = &input.dataset_description {
        object.key("datasetDescription").string(var_14.as_str());
    }
    if let Some(var_15) = &input.dataset_title {
        object.key("datasetTitle").string(var_15.as_str());
    }
    if let Some(var_16) = &input.kind {
        object.key("kind").string(var_16.as_str());
    }
    if let Some(var_17) = &input.owner_info {
        let mut object_18 = object.key("ownerInfo").start_object();
        crate::json_ser::serialize_structure_crate_model_dataset_owner_info(
            &mut object_18,
            var_17,
        )?;
        object_18.finish();
    }
    if let Some(var_19) = &input.permission_group_params {
        let mut object_20 = object.key("permissionGroupParams").start_object();
        crate::json_ser::serialize_structure_crate_model_permission_group_params(
            &mut object_20,
            var_19,
        )?;
        object_20.finish();
    }
    if let Some(var_21) = &input.schema_definition {
        let mut object_22 = object.key("schemaDefinition").start_object();
        crate::json_ser::serialize_structure_crate_model_schema_union(&mut object_22, var_21)?;
        object_22.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_create_data_view_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::CreateDataViewInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_23) = &input.as_of_timestamp {
        object.key("asOfTimestamp").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_23).into()),
        );
    }
    if input.auto_update {
        object.key("autoUpdate").boolean(input.auto_update);
    }
    if let Some(var_24) = &input.client_token {
        object.key("clientToken").string(var_24.as_str());
    }
    if let Some(var_25) = &input.destination_type_params {
        let mut object_26 = object.key("destinationTypeParams").start_object();
        crate::json_ser::serialize_structure_crate_model_data_view_destination_type_params(
            &mut object_26,
            var_25,
        )?;
        object_26.finish();
    }
    if let Some(var_27) = &input.partition_columns {
        let mut array_28 = object.key("partitionColumns").start_array();
        for item_29 in var_27 {
            {
                array_28.value().string(item_29.as_str());
            }
        }
        array_28.finish();
    }
    if let Some(var_30) = &input.sort_columns {
        let mut array_31 = object.key("sortColumns").start_array();
        for item_32 in var_30 {
            {
                array_31.value().string(item_32.as_str());
            }
        }
        array_31.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_create_permission_group_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::CreatePermissionGroupInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_33) = &input.application_permissions {
        let mut array_34 = object.key("applicationPermissions").start_array();
        for item_35 in var_33 {
            {
                array_34.value().string(item_35.as_str());
            }
        }
        array_34.finish();
    }
    if let Some(var_36) = &input.client_token {
        object.key("clientToken").string(var_36.as_str());
    }
    if let Some(var_37) = &input.description {
        object.key("description").string(var_37.as_str());
    }
    if let Some(var_38) = &input.name {
        object.key("name").string(var_38.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_create_user_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::CreateUserInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_39) = &input.api_access {
        object.key("ApiAccess").string(var_39.as_str());
    }
    if let Some(var_40) = &input.api_access_principal_arn {
        object.key("apiAccessPrincipalArn").string(var_40.as_str());
    }
    if let Some(var_41) = &input.client_token {
        object.key("clientToken").string(var_41.as_str());
    }
    if let Some(var_42) = &input.email_address {
        object.key("emailAddress").string(var_42.as_str());
    }
    if let Some(var_43) = &input.first_name {
        object.key("firstName").string(var_43.as_str());
    }
    if let Some(var_44) = &input.last_name {
        object.key("lastName").string(var_44.as_str());
    }
    if let Some(var_45) = &input.r#type {
        object.key("type").string(var_45.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_disable_user_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::DisableUserInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_46) = &input.client_token {
        object.key("clientToken").string(var_46.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_enable_user_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::EnableUserInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_47) = &input.client_token {
        object.key("clientToken").string(var_47.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_get_working_location_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::GetWorkingLocationInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_48) = &input.location_type {
        object.key("locationType").string(var_48.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_reset_user_password_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::ResetUserPasswordInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_49) = &input.client_token {
        object.key("clientToken").string(var_49.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_update_changeset_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::UpdateChangesetInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_50) = &input.client_token {
        object.key("clientToken").string(var_50.as_str());
    }
    if let Some(var_51) = &input.format_params {
        let mut object_52 = object.key("formatParams").start_object();
        for (key_53, value_54) in var_51 {
            {
                object_52.key(key_53).string(value_54.as_str());
            }
        }
        object_52.finish();
    }
    if let Some(var_55) = &input.source_params {
        let mut object_56 = object.key("sourceParams").start_object();
        for (key_57, value_58) in var_55 {
            {
                object_56.key(key_57).string(value_58.as_str());
            }
        }
        object_56.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_update_dataset_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::UpdateDatasetInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_59) = &input.alias {
        object.key("alias").string(var_59.as_str());
    }
    if let Some(var_60) = &input.client_token {
        object.key("clientToken").string(var_60.as_str());
    }
    if let Some(var_61) = &input.dataset_description {
        object.key("datasetDescription").string(var_61.as_str());
    }
    if let Some(var_62) = &input.dataset_title {
        object.key("datasetTitle").string(var_62.as_str());
    }
    if let Some(var_63) = &input.kind {
        object.key("kind").string(var_63.as_str());
    }
    if let Some(var_64) = &input.schema_definition {
        let mut object_65 = object.key("schemaDefinition").start_object();
        crate::json_ser::serialize_structure_crate_model_schema_union(&mut object_65, var_64)?;
        object_65.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_update_permission_group_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::UpdatePermissionGroupInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_66) = &input.application_permissions {
        let mut array_67 = object.key("applicationPermissions").start_array();
        for item_68 in var_66 {
            {
                array_67.value().string(item_68.as_str());
            }
        }
        array_67.finish();
    }
    if let Some(var_69) = &input.client_token {
        object.key("clientToken").string(var_69.as_str());
    }
    if let Some(var_70) = &input.description {
        object.key("description").string(var_70.as_str());
    }
    if let Some(var_71) = &input.name {
        object.key("name").string(var_71.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_update_user_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::UpdateUserInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_72) = &input.api_access {
        object.key("apiAccess").string(var_72.as_str());
    }
    if let Some(var_73) = &input.api_access_principal_arn {
        object.key("apiAccessPrincipalArn").string(var_73.as_str());
    }
    if let Some(var_74) = &input.client_token {
        object.key("clientToken").string(var_74.as_str());
    }
    if let Some(var_75) = &input.first_name {
        object.key("firstName").string(var_75.as_str());
    }
    if let Some(var_76) = &input.last_name {
        object.key("lastName").string(var_76.as_str());
    }
    if let Some(var_77) = &input.r#type {
        object.key("type").string(var_77.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_model_dataset_owner_info(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::DatasetOwnerInfo,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_78) = &input.name {
        object.key("name").string(var_78.as_str());
    }
    if let Some(var_79) = &input.phone_number {
        object.key("phoneNumber").string(var_79.as_str());
    }
    if let Some(var_80) = &input.email {
        object.key("email").string(var_80.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_model_permission_group_params(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::PermissionGroupParams,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_81) = &input.permission_group_id {
        object.key("permissionGroupId").string(var_81.as_str());
    }
    if let Some(var_82) = &input.dataset_permissions {
        let mut array_83 = object.key("datasetPermissions").start_array();
        for item_84 in var_82 {
            {
                let mut object_85 = array_83.value().start_object();
                crate::json_ser::serialize_structure_crate_model_resource_permission(
                    &mut object_85,
                    item_84,
                )?;
                object_85.finish();
            }
        }
        array_83.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_model_schema_union(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::SchemaUnion,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_86) = &input.tabular_schema_config {
        let mut object_87 = object.key("tabularSchemaConfig").start_object();
        crate::json_ser::serialize_structure_crate_model_schema_definition(&mut object_87, var_86)?;
        object_87.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_model_data_view_destination_type_params(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::DataViewDestinationTypeParams,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_88) = &input.destination_type {
        object.key("destinationType").string(var_88.as_str());
    }
    if let Some(var_89) = &input.s3_destination_export_file_format {
        object
            .key("s3DestinationExportFileFormat")
            .string(var_89.as_str());
    }
    if let Some(var_90) = &input.s3_destination_export_file_format_options {
        let mut object_91 = object
            .key("s3DestinationExportFileFormatOptions")
            .start_object();
        for (key_92, value_93) in var_90 {
            {
                object_91.key(key_92).string(value_93.as_str());
            }
        }
        object_91.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_model_resource_permission(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::ResourcePermission,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_94) = &input.permission {
        object.key("permission").string(var_94.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_model_schema_definition(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::SchemaDefinition,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_95) = &input.columns {
        let mut array_96 = object.key("columns").start_array();
        for item_97 in var_95 {
            {
                let mut object_98 = array_96.value().start_object();
                crate::json_ser::serialize_structure_crate_model_column_definition(
                    &mut object_98,
                    item_97,
                )?;
                object_98.finish();
            }
        }
        array_96.finish();
    }
    if let Some(var_99) = &input.primary_key_columns {
        let mut array_100 = object.key("primaryKeyColumns").start_array();
        for item_101 in var_99 {
            {
                array_100.value().string(item_101.as_str());
            }
        }
        array_100.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_model_column_definition(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::ColumnDefinition,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_102) = &input.data_type {
        object.key("dataType").string(var_102.as_str());
    }
    if let Some(var_103) = &input.column_name {
        object.key("columnName").string(var_103.as_str());
    }
    if let Some(var_104) = &input.column_description {
        object.key("columnDescription").string(var_104.as_str());
    }
    Ok(())
}
