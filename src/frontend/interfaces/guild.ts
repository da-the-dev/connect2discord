export default interface Guild {
    id: string
    name: string
    icon: string | null
    icon_hash?: string | null
    description: string | null
    splash: string | null
    discovery_splash: string | null
    features: [string] | []
    banner: string | null
    owner?: boolean
    owner_id: string
    application_id: string | null
    permissons?: string
    region?: string | null
    afk_channel_id: string | null
    afk_timeout: string
    widget_enabled?: boolean
    widget_channel_id?: string | null
    system_channel_id: string | null
    verification_level: number
    roles: [any] | []
    emojis: [any] | []
    default_message_notifications: number
    mfa_level: number
    explicit_content_filter: number
    max_presences?: number | null
    max_members?: number
    vanity_url_code: string | null
    premium_tier: number
    premium_subscription_count?: number
    system_channel_flags: string | null
    preferred_locale: string
    rules_channel_id: string | null
    public_updates_channel_id: string | null
}
