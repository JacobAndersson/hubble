-- This file should undo anything in `up.sql`
ALTER TABLE games 
  DROP COLUMN middle_game,
  DROP COLUMN end_game
