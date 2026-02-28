-- Add migration script here
CREATE TABLE flashcards (
  id integer primary key autoincrement,
  front_of_card TEXT,
  back_of_card TEXT
  -- PRIMARY KEY(id)
)
