# Fn statement.
  
  `fn IDENT = "STRING"`

# Extern fn statement.

  `extern FFI_LANGUAGE SIGNATURE`

  *FFI_LANGUAGE* ::= {
  
      clang -- The C programming language
  
  }

  *SIGNATURE* ::= `fn IDENT($( ARG ),*) $( -> TYPE )?`

  *ARG* ::= `TYPED_VARIABLES` | `TYPE $( x UINTEGER )?`

  *TYPED_VARIABLES* ::= `$( IDENT )* : TYPE`

  *TYPE* ::= `$( POINTER )* IDENT`

  *POINTER* = `* $( mut )?`

# Type statement.

  `ty IDENT = TYPE_BODY`

  *TYPE_BODY* ::= `ENUM_TYPE_BODY` | `STRUCT_TYPE_BODY`

  *ENUM_TYPE_BODY* ::= `$( IDENT $( TYPE )? )|+`

  *STRUCT_TYPE_BODY* ::= `$( TYPED_VARIABLES )+*`
