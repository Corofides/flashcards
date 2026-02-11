-- Add migration script here
CREATE TABLE flashcards (
  id int NOT NULL,
  front_of_card TEXT,
  back_of_card TEXT,
  PRIMARY KEY(id)
)
