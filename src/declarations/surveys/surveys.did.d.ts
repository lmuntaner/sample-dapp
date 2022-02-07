import type { Principal } from '@dfinity/principal';
export interface Question { 'title' : string }
export interface Survey {
  'id' : bigint,
  'title' : string,
  'questions' : Array<Question>,
}
export interface _SERVICE {
  'create' : (arg_0: string, arg_1: Array<string>) => Promise<Survey>,
  'read_all' : () => Promise<Array<Survey>>,
}
