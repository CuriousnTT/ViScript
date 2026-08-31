const boundary1= [|1|];

const boundary2=([|1|]);

const boundary3={[|1|]};

const boundary4=[[|1|]];

const boundary5=foo([|1|]);

const boundary6=foo([|1|],[|2|]);

const boundary7=([|1|],[|2|]);

const boundary8=[|([1]),({value: 2}),foo(3)|];

const finalSet = [|"this set is at the end of a line"|];