-- Define arrays for adjectives and nouns
DO $$
BEGIN
  CREATE TEMP TABLE temp_adjectives (word VARCHAR(50)) ON COMMIT DROP;
  CREATE TEMP TABLE temp_elements (word VARCHAR(50)) ON COMMIT DROP;
  CREATE TEMP TABLE temp_nouns (word VARCHAR(50)) ON COMMIT DROP;
  INSERT INTO temp_adjectives (word) VALUES
  ('Frosty'),
  ('Burning'),
  ('Electric'),
  ('Toxic'),
  ('Celestial'),
  ('Shadowy'),
  ('Mystical'),
  ('Radiant'),
  ('Solar'),
  ('Lunar'),
  ('Howling'),
  ('Waving'),
  ('Frightful'),
  ('Unyielding'),
  ('Relentless'),
  ('Dying'),
  ('Stupifying'),
  ('Searing'),
  ('Torturous'),
  ('Lacerating'),
  ('Patient'),
  ('Venemous'),
  ('Bilious'),
  ('Suffering'),
  ('Drunken'),
  ('Explosive'),
  ('Sharpened'),
  ('Celestial'),
  ('Scorching');

  INSERT INTO temp_elements (word) VALUES
  ('Fire'),
  ('Dark'),
  ('Frost'),
  ('Poison'),
  ('Swamp'),
  ('Sticking'),
  ('Dry'),
  ('Shocking'),
  ('Gravitational'),
  ('Frozen'),
  ('Kinetic'),
  ('Earth'),
  ('Electric'),
  ('Sand'),
  ('Wind'),
  ('Plant'),
  ('Grass'),
  ('Rocky'),
  ('Water'),
  ('Holy'),
  ('Heavenly'),
  ('Cursed'),
  ('Broken'),
  ('Chaos'),
  ('Lava'),
  ('Dragon'),
  ('Ice'),
  ('Gas'),
  ('Corrupt');

  INSERT INTO temp_nouns (word) VALUES
  ('Storm'),
  ('Smash'),
  ('Bolt'),
  ('Knife'),
  ('Bullet'),
  ('Blade'),
  ('Sword'),
  ('Axe'),
  ('Wave'),
  ('Flame'),
  ('Shard'),
  ('Push'),
  ('Pull'),
  ('Whirlwind'),
  ('Tornado'),
  ('Hurricane'),
  ('Tsunami'),
  ('Nova'),
  ('Flare'),
  ('Vortex'),
  ('Fury'),
  ('Howl'),
  ('Cinder'),
  ('Ember'),
  ('Scourge'),
  ('Ghost'),
  ('Bite'),
  ('Clap'),
  ('Volcano'),
  ('Screech'),
  ('Cut'),
  ('Touch'),
  ('Scratch'),
  ('Poke'),
  ('Lament'),
  ('Curse');
END $$;

-- Create function to generate random spell names
CREATE OR REPLACE FUNCTION generate_spell_name()
RETURNS VARCHAR AS
$$
DECLARE
  adj VARCHAR;
  noun VARCHAR;
  verb VARCHAR;
BEGIN
  SELECT word INTO adj FROM (SELECT unnest(array_agg(word)) as word FROM temp_adjectives) AS adj_words ORDER BY RANDOM() LIMIT 1;
  SELECT word INTO verb FROM (SELECT unnest(array_agg(word)) as word FROM temp_elements) AS elements ORDER BY RANDOM() LIMIT 1;
  SELECT word INTO noun FROM (SELECT unnest(array_agg(word)) as word FROM temp_nouns) AS noun_words ORDER BY RANDOM() LIMIT 1;

  RETURN adj || ' ' || verb || ' ' || noun;
END;
$$
LANGUAGE PLPGSQL;

-- Insert random spells
DO $$
BEGIN
  FOR i IN 1..1000000 LOOP
    INSERT INTO spell (name, damage)
    VALUES (generate_spell_name(),FLOOR(RANDOM() * 1000000) + 1);
  END LOOP;
END;
$$;

-- Drop
DROP FUNCTION IF EXISTS generate_spell_name();
