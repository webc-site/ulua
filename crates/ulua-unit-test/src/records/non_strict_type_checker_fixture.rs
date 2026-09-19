use alloc::string::String;

use crate::records::fixture::Fixture;

#[derive(Debug)]
pub struct NonStrictTypeCheckerFixture {
  pub base: Fixture,
  pub definitions: String,
}

impl Default for NonStrictTypeCheckerFixture {
  fn default() -> Self {
    Self {
      base: Fixture::fixture_bool(false),
      definitions: String::from(
        "@checked declare function abs(n: number): number\n\
                 @checked declare function lower(s: string): string\n\
                 declare function cond() : boolean\n\
                 @checked declare function contrived(n : Not<number>) : number\n\
                 \n\
                 -- interesting types of things that we would like to mark as checked\n\
                 @checked declare function onlyNums(...: number) : number\n\
                 @checked declare function mixedArgs(x: string, ...: number) : number\n\
                 @checked declare function optionalArg(x: string?) : number\n\
                 declare foo: {\n\
                     bar: @checked (number) -> number,\n\
                 }\n\
                 \n\
                 @checked declare function optionalArgsAtTheEnd1(x: string, y: number?, z: number?) : number\n\
                 @checked declare function optionalArgsAtTheEnd2(x: string, y: number?, z: string) : number\n\
                 \n\
                 type DateTypeArg = {\n\
                     year: number,\n\
                     month: number,\n\
                     day: number,\n\
                     hour: number?,\n\
                     min: number?,\n\
                     sec: number?,\n\
                     isdst: boolean?,\n\
                 }\n\
                 \n\
                 declare os : {\n\
                     time: @checked (time: DateTypeArg?) -> number\n\
                 }\n\
                 \n\
                 @checked declare function require(target : any) : any\n\
                 @checked declare function getAllTheArgsWrong(one: string, two: number, three: boolean) : any\n",
      ),
    }
  }
}
