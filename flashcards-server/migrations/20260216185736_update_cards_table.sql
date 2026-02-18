--- Add migration script here
ALTER TABLE flashcards 
  ADD COLUMN ease_factor int DEFAULT 3;

ALTER TABLE flashcards
  ADD COLUMN interval int DEFAULT 1;

ALTER TABLE flashcards
  ADD COLUMN next_review TIMESTAMP DEFAULT CURRENT_TIMESTAMP;
