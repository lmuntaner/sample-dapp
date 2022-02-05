export const idlFactory = ({ IDL }) => {
  return IDL.Service({
    'inc' : IDL.Func([], [], []),
    'read' : IDL.Func([], [IDL.Nat], ['query']),
    'start' : IDL.Func([], [], []),
    'write' : IDL.Func([IDL.Nat], [], []),
  });
};
export const init = ({ IDL }) => { return []; };
