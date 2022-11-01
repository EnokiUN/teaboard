CREATE TABLE IF NOT EXISTS posts (
	id BIGINT PRIMARY KEY,
	board VARCHAR(32) NOT NULL,
	title VARCHAR(256) NOT NULL,
	content VARCHAR(10000),
	pinned boolean NOT NULL DEFAULT false,
	moderator boolean NOT NULL DEFAULT false,
	locked boolean NOT NULL DEFAULT false,
	parent BIGINT,
	image BIGINT,
	FOREIGN KEY (board) REFERENCES boards(id),
	FOREIGN KEY (image) REFERENCES images(id),
	FOREIGN KEY (parent) REFERENCES posts(id)
);
