-- Your SQL goes here
CREATE TABLE users (
	id serial NOT NULL PRIMARY key,
	name TEXT NOT NULL,
	UNIQUE (name)
);

CREATE TABLE posts (
	id serial NOT NULL PRIMARY key,
	user_id INT NOT NULL,
	title VARCHAR(200) NOT NULL,
	CONSTRAINT fk_user
      FOREIGN KEY(user_id) 
	  REFERENCES users(id)
);