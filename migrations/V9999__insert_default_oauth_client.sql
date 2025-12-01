INSERT INTO oauth_client (
    "name",
    "scopes",
    "mandatory_scopes",
    "slug",
    "status",
    "urls"
) VALUES (
    'Amaterasu',
    '{"openid", "profile", "email", "offline_access"}',
    '{"openid", "email", "offline_access"}',
    'amaterasu',
    1,
    '{"http://localhost:3000/callback"}'
);