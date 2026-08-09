export type ConfigEntry = {
  key: string;
  value: string;
};

export type ConfigSection = {
  name: string;
  entries: ConfigEntry[];
};

export type AppConfigData = {
  sections: ConfigSection[];
  config_path: string;
};

export type SaveAppConfigRequest = {
  sections: ConfigSection[];
};
