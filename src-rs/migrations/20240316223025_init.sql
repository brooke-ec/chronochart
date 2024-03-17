-- Setup metadata table
CREATE TABLE [metadata] ( 
  [key] TEXT NOT NULL,
  [value] TEXT NULL,
   PRIMARY KEY ([key])
);

-- Setup event table
CREATE TABLE [event] ( 
  [uuid] TEXT NOT NULL,
  [timestamp] INTEGER NOT NULL,
  [title] TEXT NOT NULL,
   PRIMARY KEY ([uuid])
);

CREATE INDEX [IX_event_timestamp] 
ON [event] (
  [timestamp] ASC
);
