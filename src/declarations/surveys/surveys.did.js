export const idlFactory = ({ IDL }) => {
  const Question = IDL.Record({ 'title' : IDL.Text });
  const Survey = IDL.Record({
    'id' : IDL.Text,
    'title' : IDL.Text,
    'questions' : IDL.Vec(Question),
  });
  return IDL.Service({
    'create' : IDL.Func([IDL.Text, IDL.Vec(IDL.Text)], [Survey], []),
    'read_all' : IDL.Func([], [IDL.Vec(Survey)], ['query']),
  });
};
export const init = ({ IDL }) => { return []; };
