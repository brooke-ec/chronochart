-- Setup metadata table
CREATE TABLE [metadata] ( 
  [key] TEXT NOT NULL,
  [value] TEXT NULL,
   PRIMARY KEY ([key])
);

-- Setup timeline table
CREATE TABLE [timeline] ( 
  [uuid] TEXT NOT NULL,
  [title] TEXT NOT NULL,
  [color] TEXT NOT NULL,
  [parent_uuid] TEXT NULL,
   PRIMARY KEY ([uuid]),
   FOREIGN KEY ([parent_uuid]) REFERENCES [timeline]([uuid]) ON DELETE SET NULL
);

-- Setup event table
CREATE TABLE [event] ( 
  [uuid] TEXT NOT NULL,
  [timestamp] INTEGER NOT NULL,
  [color] TEXT NOT NULL,
  [title] TEXT NOT NULL,
   PRIMARY KEY ([uuid])
);

CREATE INDEX [IX_event_timestamp] 
ON [event] (
  [timestamp] ASC
);

-- Setup event/timeline many-to-many relationship
CREATE TABLE [event_timeline] (
  [event_uuid] TEXT NOT NULL,
  [timeline_uuid] TEXT NOT NULL,
   PRIMARY KEY ([event_uuid], [timeline_uuid]),
   FOREIGN KEY ([event_uuid]) REFERENCES [event]([uuid]) ON DELETE CASCADE,
   FOREIGN KEY ([timeline_uuid]) REFERENCES [timeline]([uuid]) ON DELETE CASCADE
)

