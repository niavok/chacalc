Chacacl is a scientific oriented advanced calculator.

The goal is to provide the following features :


- simple calculator
- advanded calculator
- units awareness
- time operations
- angle, vector, rotation, quaternion, matrix operations
- variables
- universial constants
- fonctions
- pratical interface : tab completion, helper, sessions, projects and history
- plot interactive graph
- etc

General Usage
=============

Varius string are reserved keyword not usable as variable
# character delimit comments

Command
=======

help <variable|unit|operator|constant>
    give help for the given variable, command, operator or constant

list-variables
list-commands
list-constants
delete <variable>
show <variable>
set-default-unit <unit>
    use this unit as default unit for it corresponding physical quantity
define-unit <unit> <value>
flat <expression>
    print the operation replacing constant and variable by theirs values
plot ...
set-presicion decimal precision

Constants
=========

G
c 300000 km.s-1

Units
=====

s, minutes, hour, etc
m
kg
g  gram
g0 9.81m.s-2

Operator
========

+
-
/
*
mod(
sin(
cos(


in
    5kg in grams -> 5000g

Value
=====

a quantity and an optionnal unit.
there is no space between the quantity and the unit
there is always a space between the unit and the operator

-

(5 + 3 + 12 + 8)kg  -> 28kg
