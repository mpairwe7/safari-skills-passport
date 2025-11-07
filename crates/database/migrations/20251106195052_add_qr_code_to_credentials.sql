-- Add qr_code column to credentials table
ALTER TABLE credentials ADD COLUMN qr_code TEXT NOT NULL DEFAULT '';