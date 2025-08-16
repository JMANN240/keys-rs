CREATE TABLE api_keys (
	api_key_hash_base64 TEXT PRIMARY KEY NOT NULL
);

CREATE TABLE key_values (
	api_key_hash_base64 TEXT NOT NULL,
	key TEXT NOT NULL,
	cipher_value_base64 TEXT NOT NULL,
	nonce_base64 TEXT NOT NULL,
	PRIMARY KEY (api_key_hash_base64, key),
	FOREIGN KEY (api_key_hash_base64) REFERENCES api_keys (api_key_hash_base64)
);
