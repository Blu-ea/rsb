/*
    AB&! -> A!B!|   // Put the ! close to the numbers
    AB|! -> A!B!&   //  ^ same here

    AB>  ->   A!B|        // just a conversion
    AB=  -> AB&A!B!&|   // Just a conversion
-
    How to have the NNF
    -> remove all the unwanted notation (=,>,^)
    -> put the ! sign just next to the variables and if an other ! sign is found, they cancel ou, if a & or | is found, they swap
*/
use rsb::ex05::negation_normal_form;

#[test]
fn test_ex05(){
    negation_normal_form("AB&AB|AB|AB||||");
}