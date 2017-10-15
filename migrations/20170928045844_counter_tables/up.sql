-- Create a counters table

CREATE TABLE "counters" (
  "id"   serial NOT NULL PRIMARY KEY,
  "url"  character varying(64) NOT NULL,
  "name" character varying(64) NOT NULL,
  "unit" character varying(64) NOT NULL
);

COMMENT ON TABLE "counters" IS 'Properties of each counter a user has set up';
COMMENT ON COLUMN "counters"."id" IS 'Unique Identifier for each counter';
COMMENT ON COLUMN "counters"."url" IS 'URL-friendly portion of the "name" of this counter';
COMMENT ON COLUMN "counters"."name" IS 'Human-friendly name for this counter';
COMMENT ON COLUMN "counters"."unit" IS 'The unit of each count';

-- Create a table for each counted event 

CREATE TABLE "counter_events" (
    "id" serial NOT NULL PRIMARY KEY,
    "cid" int NOT NULL REFERENCES "counters" ("id"),
    "quantity" int NOT NULL,
    "timestamp" timestamptz NOT NULL
);

COMMENT ON TABLE "counter_events" IS 'Record of each event and associated quantity for each counter in the "counters" table';
COMMENT ON COLUMN "counter_events"."id" IS 'Unique event identifier';
COMMENT ON COLUMN "counter_events"."cid" IS 'ID of the counter in which this event is being recorded';
COMMENT ON COLUMN "counter_events"."quantity" IS 'The quantity to be counted for this event';
COMMENT ON COLUMN "counter_events"."timestamp" IS 'The time at which the user states this event occurred';