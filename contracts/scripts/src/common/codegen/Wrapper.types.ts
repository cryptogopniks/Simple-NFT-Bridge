/**
* This file was automatically generated by @cosmwasm/ts-codegen@1.9.0.
* DO NOT MODIFY IT BY HAND. Instead, modify the source JSONSchema file,
* and run the @cosmwasm/ts-codegen generate command to regenerate this file.
*/

export interface InstantiateMsg {
  lending_platform: string;
  nft_minter: string;
  worker?: string | null;
}
export type ExecuteMsg = {
  accept_admin_role: {};
} | {
  update_config: {
    admin?: string | null;
    worker?: string | null;
  };
} | {
  pause: {};
} | {
  unpause: {};
} | {
  wrap: {
    collection_in: string;
    token_list: string[];
  };
} | {
  unwrap: {
    collection_out: string;
    token_list: string[];
  };
} | {
  add_collection: {
    collection_in: string;
    collection_out: string;
  };
} | {
  remove_collection: {
    collection_in: string;
  };
};
export type QueryMsg = {
  config: {};
} | {
  collection_list: {};
} | {
  collection: {
    collection_in: string;
  };
};
export interface MigrateMsg {
  version: string;
}
export type Addr = string;
export interface Collection {
  collection_in: Addr;
  collection_out: Addr;
}
export type ArrayOfCollection = Collection[];
export interface Config {
  admin: Addr;
  lending_platform: Addr;
  nft_minter: Addr;
  worker?: Addr | null;
}