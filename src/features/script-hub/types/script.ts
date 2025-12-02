export interface Script {
  _id: string;
  title: string;
  game: {
    _id: string;
    name: string;
    imageUrl: string;
  };
  slug: string;
  verified: boolean;
  key: boolean;
  views: number;
  scriptType: "free" | "paid";
  isUniversal: boolean;
  isPatched: boolean;
  image: string;
  lastBump?: string;
  createdAt: string;
  updatedAt: string;
  script: string;
  matched?: string[];
}

export interface ScriptFetchResponse {
  result: {
    totalPages: number;
    nextPage: number;
    max: number;
    scripts: Script[];
  };
}

export interface ScriptSearchResponse {
  result: {
    totalPages: number;
    scripts: Script[];
  };
}

export type ScriptMode = "free" | "paid";
export type SortBy =
  | "views"
  | "likeCount"
  | "createdAt"
  | "updatedAt"
  | "dislikeCount"
  | "accuracy";
export type SortOrder = "asc" | "desc";

export interface ScriptSearchParams {
  q?: string;
  page?: number;
  max?: number;
  mode?: ScriptMode;
  patched?: 0 | 1;
  key?: 0 | 1;
  universal?: 0 | 1;
  verified?: 0 | 1;
  sortBy?: SortBy;
  order?: SortOrder;
  strict?: boolean;
  owner?: string;
  placeId?: number;
}
