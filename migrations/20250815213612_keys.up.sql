CREATE TABLE key_stores (
	store TEXT PRIMARY KEY NOT NULL
);

CREATE TABLE key_values (
	store TEXT NOT NULL,
	key TEXT NOT NULL,
	value TEXT NOT NULL,
	PRIMARY KEY (store, key),
	FOREIGN KEY (store) REFERENCES key_stores (store)
);
