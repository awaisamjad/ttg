# To Do

## Checking validity of inputs
- Is it possible to have a general set of syntax that is allowed for a statement, operation etc and see if the input fits the criteria/matches that syntax
- Example for statement
  - Syntax : NAME = [value1, value2, ...]
- Example for operations
  - Syntax 1 : NAME
  - Syntax 2 : (OPERATION)NAME
  - Syntax 3 : NAME OPERATION NAME
## Features
- Equal testing
  - Could be done with PartialEq trait

## File structure
- src
  - repl
  - 

## Creating a statement
- NAME = [value1, value2, ...];
  - Example : p1 = [t, t, f, f];

#### Rules
1. Values can be either t/true and f/false only.
   1. Case INSENSITIVE
2. Equals sign must be used
3. Name MUST be only mixture of letters and numbers
   1. NO special characters except for underscore and hyphen (easier reading)
4. Line must end in semi colon or will continue to line break until finished

## Valid Operations
OR : v, |, OR, or 
AND: ∧ , &, AND, and
NOT: ¬ , !, NOT, not
XOR : X, XOR, xor
XNOR : ¬X, !X, XNOR, xnor
IF THEN : ->, =>, IF-THEN, if-then
BICONDITIONAL : <->, <=>, ????????????

### Valid Syntax
==ASSUMES ALL STATEMENTS ARE DEFINED PREVIOUSLY==
#### Using a singular statement
- p1;

#### Performing operation on single statement
- ¬ p1; 
- ! p1;

> Only includes NOT

#### Defining statements
- [t,t,t,f];
- [t,t,t,t,t,t,t,f,f,f,f,f];

#### Creating Variables
- p1 = [t,t,t,t];
- p2 = [f,f,f,f];

#### Using 2 statements
- p1; p2;

#### Performing operations on 2 statements 
- p1 ^ p2;

#### Creating Basic variables from others variables using operations
- p3 = p1 ^ p2;

#### Multiple operations 
##### NO Bracket operations are performed consequently
- p1 ^ p2 v p3 ^ p4 = (((p1 ^ p2) v p3) ^ p4)
- These wont require anything special as they are performed one after the next
  

##### Bracket operations are performed in hierarchy of brackets
- ((p1 ^ p2) v (p3 ^ p4)) v p5 =
  1. p1 ^ p2
  2. p3 ^ p4
  3. (p1 ^ p2) v (p3 ^ p4)
  4. ((p1 ^ p2) v (p3 ^ p4))v p5
- these would require an operation to `unwrap` the brackets

#### Overriding a value
- p1 = [t,t];
- p1 = [f,f];
- p1 is equal to [f,f] and not [t,t]
- 