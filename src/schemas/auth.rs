// @generated automatically by Diesel CLI.

pub mod auth {
    pub mod sql_types {
        #[derive(diesel::query_builder::QueryId, Clone, diesel::sql_types::SqlType)]
        #[diesel(postgres_type(name = "aal_level", schema = "auth"))]
        pub struct AalLevel;

        #[derive(diesel::query_builder::QueryId, Clone, diesel::sql_types::SqlType)]
        #[diesel(postgres_type(name = "code_challenge_method", schema = "auth"))]
        pub struct CodeChallengeMethod;

        #[derive(diesel::query_builder::QueryId, Clone, diesel::sql_types::SqlType)]
        #[diesel(postgres_type(name = "factor_status", schema = "auth"))]
        pub struct FactorStatus;

        #[derive(diesel::query_builder::QueryId, Clone, diesel::sql_types::SqlType)]
        #[diesel(postgres_type(name = "factor_type", schema = "auth"))]
        pub struct FactorType;

        #[derive(diesel::query_builder::QueryId, Clone, diesel::sql_types::SqlType)]
        #[diesel(postgres_type(name = "one_time_token_type", schema = "auth"))]
        pub struct OneTimeTokenType;
    }

    diesel::table! {
        auth.audit_log_entries (id) {
            instance_id -> Nullable<Uuid>,
            id -> Uuid,
            payload -> Nullable<Json>,
            created_at -> Nullable<Timestamptz>,
            #[max_length = 64]
            ip_address -> Varchar,
        }
    }

    diesel::table! {
        use diesel::sql_types::*;
        use super::sql_types::CodeChallengeMethod;

        auth.flow_state (id) {
            id -> Uuid,
            user_id -> Nullable<Uuid>,
            auth_code -> Text,
            code_challenge_method -> CodeChallengeMethod,
            code_challenge -> Text,
            provider_type -> Text,
            provider_access_token -> Nullable<Text>,
            provider_refresh_token -> Nullable<Text>,
            created_at -> Nullable<Timestamptz>,
            updated_at -> Nullable<Timestamptz>,
            authentication_method -> Text,
            auth_code_issued_at -> Nullable<Timestamptz>,
        }
    }

    diesel::table! {
        auth.identities (id) {
            provider_id -> Text,
            user_id -> Uuid,
            identity_data -> Jsonb,
            provider -> Text,
            last_sign_in_at -> Nullable<Timestamptz>,
            created_at -> Nullable<Timestamptz>,
            updated_at -> Nullable<Timestamptz>,
            email -> Nullable<Text>,
            id -> Uuid,
        }
    }

    diesel::table! {
        auth.instances (id) {
            id -> Uuid,
            uuid -> Nullable<Uuid>,
            raw_base_config -> Nullable<Text>,
            created_at -> Nullable<Timestamptz>,
            updated_at -> Nullable<Timestamptz>,
        }
    }

    diesel::table! {
        auth.mfa_amr_claims (id) {
            session_id -> Uuid,
            created_at -> Timestamptz,
            updated_at -> Timestamptz,
            authentication_method -> Text,
            id -> Uuid,
        }
    }

    diesel::table! {
        auth.mfa_challenges (id) {
            id -> Uuid,
            factor_id -> Uuid,
            created_at -> Timestamptz,
            verified_at -> Nullable<Timestamptz>,
            ip_address -> Inet,
            otp_code -> Nullable<Text>,
            web_authn_session_data -> Nullable<Jsonb>,
        }
    }

    diesel::table! {
        use diesel::sql_types::*;
        use super::sql_types::FactorType;
        use super::sql_types::FactorStatus;

        auth.mfa_factors (id) {
            id -> Uuid,
            user_id -> Uuid,
            friendly_name -> Nullable<Text>,
            factor_type -> FactorType,
            status -> FactorStatus,
            created_at -> Timestamptz,
            updated_at -> Timestamptz,
            secret -> Nullable<Text>,
            phone -> Nullable<Text>,
            last_challenged_at -> Nullable<Timestamptz>,
            web_authn_credential -> Nullable<Jsonb>,
            web_authn_aaguid -> Nullable<Uuid>,
        }
    }

    diesel::table! {
        use diesel::sql_types::*;
        use super::sql_types::OneTimeTokenType;

        auth.one_time_tokens (id) {
            id -> Uuid,
            user_id -> Uuid,
            token_type -> OneTimeTokenType,
            token_hash -> Text,
            relates_to -> Text,
            created_at -> Timestamp,
            updated_at -> Timestamp,
        }
    }

    diesel::table! {
        auth.refresh_tokens (id) {
            instance_id -> Nullable<Uuid>,
            id -> Int8,
            #[max_length = 255]
            token -> Nullable<Varchar>,
            #[max_length = 255]
            user_id -> Nullable<Varchar>,
            revoked -> Nullable<Bool>,
            created_at -> Nullable<Timestamptz>,
            updated_at -> Nullable<Timestamptz>,
            #[max_length = 255]
            parent -> Nullable<Varchar>,
            session_id -> Nullable<Uuid>,
        }
    }

    diesel::table! {
        auth.saml_providers (id) {
            id -> Uuid,
            sso_provider_id -> Uuid,
            entity_id -> Text,
            metadata_xml -> Text,
            metadata_url -> Nullable<Text>,
            attribute_mapping -> Nullable<Jsonb>,
            created_at -> Nullable<Timestamptz>,
            updated_at -> Nullable<Timestamptz>,
            name_id_format -> Nullable<Text>,
        }
    }

    diesel::table! {
        auth.saml_relay_states (id) {
            id -> Uuid,
            sso_provider_id -> Uuid,
            request_id -> Text,
            for_email -> Nullable<Text>,
            redirect_to -> Nullable<Text>,
            created_at -> Nullable<Timestamptz>,
            updated_at -> Nullable<Timestamptz>,
            flow_state_id -> Nullable<Uuid>,
        }
    }

    diesel::table! {
        auth.schema_migrations (version) {
            #[max_length = 255]
            version -> Varchar,
        }
    }

    diesel::table! {
        use diesel::sql_types::*;
        use super::sql_types::AalLevel;

        auth.sessions (id) {
            id -> Uuid,
            user_id -> Uuid,
            created_at -> Nullable<Timestamptz>,
            updated_at -> Nullable<Timestamptz>,
            factor_id -> Nullable<Uuid>,
            aal -> Nullable<AalLevel>,
            not_after -> Nullable<Timestamptz>,
            refreshed_at -> Nullable<Timestamp>,
            user_agent -> Nullable<Text>,
            ip -> Nullable<Inet>,
            tag -> Nullable<Text>,
        }
    }

    diesel::table! {
        auth.sso_domains (id) {
            id -> Uuid,
            sso_provider_id -> Uuid,
            domain -> Text,
            created_at -> Nullable<Timestamptz>,
            updated_at -> Nullable<Timestamptz>,
        }
    }

    diesel::table! {
        auth.sso_providers (id) {
            id -> Uuid,
            resource_id -> Nullable<Text>,
            created_at -> Nullable<Timestamptz>,
            updated_at -> Nullable<Timestamptz>,
        }
    }

    diesel::table! {
        auth.users (id) {
            instance_id -> Nullable<Uuid>,
            id -> Uuid,
            #[max_length = 255]
            aud -> Nullable<Varchar>,
            #[max_length = 255]
            role -> Nullable<Varchar>,
            #[max_length = 255]
            email -> Nullable<Varchar>,
            #[max_length = 255]
            encrypted_password -> Nullable<Varchar>,
            email_confirmed_at -> Nullable<Timestamptz>,
            invited_at -> Nullable<Timestamptz>,
            #[max_length = 255]
            confirmation_token -> Nullable<Varchar>,
            confirmation_sent_at -> Nullable<Timestamptz>,
            #[max_length = 255]
            recovery_token -> Nullable<Varchar>,
            recovery_sent_at -> Nullable<Timestamptz>,
            #[max_length = 255]
            email_change_token_new -> Nullable<Varchar>,
            #[max_length = 255]
            email_change -> Nullable<Varchar>,
            email_change_sent_at -> Nullable<Timestamptz>,
            last_sign_in_at -> Nullable<Timestamptz>,
            raw_app_meta_data -> Nullable<Jsonb>,
            raw_user_meta_data -> Nullable<Jsonb>,
            is_super_admin -> Nullable<Bool>,
            created_at -> Nullable<Timestamptz>,
            updated_at -> Nullable<Timestamptz>,
            phone -> Nullable<Text>,
            phone_confirmed_at -> Nullable<Timestamptz>,
            phone_change -> Nullable<Text>,
            #[max_length = 255]
            phone_change_token -> Nullable<Varchar>,
            phone_change_sent_at -> Nullable<Timestamptz>,
            confirmed_at -> Nullable<Timestamptz>,
            #[max_length = 255]
            email_change_token_current -> Nullable<Varchar>,
            email_change_confirm_status -> Nullable<Int2>,
            banned_until -> Nullable<Timestamptz>,
            #[max_length = 255]
            reauthentication_token -> Nullable<Varchar>,
            reauthentication_sent_at -> Nullable<Timestamptz>,
            is_sso_user -> Bool,
            deleted_at -> Nullable<Timestamptz>,
            is_anonymous -> Bool,
        }
    }

    diesel::joinable!(identities -> users (user_id));
    diesel::joinable!(mfa_amr_claims -> sessions (session_id));
    diesel::joinable!(mfa_challenges -> mfa_factors (factor_id));
    diesel::joinable!(mfa_factors -> users (user_id));
    diesel::joinable!(one_time_tokens -> users (user_id));
    diesel::joinable!(refresh_tokens -> sessions (session_id));
    diesel::joinable!(saml_providers -> sso_providers (sso_provider_id));
    diesel::joinable!(saml_relay_states -> flow_state (flow_state_id));
    diesel::joinable!(saml_relay_states -> sso_providers (sso_provider_id));
    diesel::joinable!(sessions -> users (user_id));
    diesel::joinable!(sso_domains -> sso_providers (sso_provider_id));

    diesel::allow_tables_to_appear_in_same_query!(
        audit_log_entries,
        flow_state,
        identities,
        instances,
        mfa_amr_claims,
        mfa_challenges,
        mfa_factors,
        one_time_tokens,
        refresh_tokens,
        saml_providers,
        saml_relay_states,
        schema_migrations,
        sessions,
        sso_domains,
        sso_providers,
        users,
    );
}
