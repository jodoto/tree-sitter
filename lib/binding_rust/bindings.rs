/* automatically generated by rust-bindgen */

pub type __darwin_size_t = ::std::os::raw::c_ulong;
pub type FILE = [u64; 19usize];
pub type TSSymbol = u16;
pub type TSFieldId = u16;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct TSLanguage {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct TSParser {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct TSTree {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct TSQuery {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct TSQueryContext {
    _unused: [u8; 0],
}
pub const TSInputEncoding_TSInputEncodingUTF8: TSInputEncoding = 0;
pub const TSInputEncoding_TSInputEncodingUTF16: TSInputEncoding = 1;
pub type TSInputEncoding = u32;
pub const TSSymbolType_TSSymbolTypeRegular: TSSymbolType = 0;
pub const TSSymbolType_TSSymbolTypeAnonymous: TSSymbolType = 1;
pub const TSSymbolType_TSSymbolTypeAuxiliary: TSSymbolType = 2;
pub type TSSymbolType = u32;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct TSPoint {
    pub row: u32,
    pub column: u32,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct TSRange {
    pub start_point: TSPoint,
    pub end_point: TSPoint,
    pub start_byte: u32,
    pub end_byte: u32,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct TSInput {
    pub payload: *mut ::std::os::raw::c_void,
    pub read: ::std::option::Option<
        unsafe extern "C" fn(
            payload: *mut ::std::os::raw::c_void,
            byte_index: u32,
            position: TSPoint,
            bytes_read: *mut u32,
        ) -> *const ::std::os::raw::c_char,
    >,
    pub encoding: TSInputEncoding,
}
pub const TSLogType_TSLogTypeParse: TSLogType = 0;
pub const TSLogType_TSLogTypeLex: TSLogType = 1;
pub type TSLogType = u32;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct TSLogger {
    pub payload: *mut ::std::os::raw::c_void,
    pub log: ::std::option::Option<
        unsafe extern "C" fn(
            payload: *mut ::std::os::raw::c_void,
            arg1: TSLogType,
            arg2: *const ::std::os::raw::c_char,
        ),
    >,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct TSInputEdit {
    pub start_byte: u32,
    pub old_end_byte: u32,
    pub new_end_byte: u32,
    pub start_point: TSPoint,
    pub old_end_point: TSPoint,
    pub new_end_point: TSPoint,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct TSNode {
    pub context: [u32; 4usize],
    pub id: *const ::std::os::raw::c_void,
    pub tree: *const TSTree,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct TSTreeCursor {
    pub tree: *const ::std::os::raw::c_void,
    pub id: *const ::std::os::raw::c_void,
    pub context: [u32; 2usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct TSQueryCapture {
    pub node: TSNode,
    pub index: u32,
}
pub const TSQueryError_TSQueryErrorNone: TSQueryError = 0;
pub const TSQueryError_TSQueryErrorSyntax: TSQueryError = 1;
pub const TSQueryError_TSQueryErrorNodeType: TSQueryError = 2;
pub const TSQueryError_TSQueryErrorField: TSQueryError = 3;
pub type TSQueryError = u32;
extern "C" {
    #[doc = " Create a new parser."]
    pub fn ts_parser_new() -> *mut TSParser;
}
extern "C" {
    #[doc = " Delete the parser, freeing all of the memory that it used."]
    pub fn ts_parser_delete(parser: *mut TSParser);
}
extern "C" {
    #[doc = " Set the language that the parser should use for parsing."]
    #[doc = ""]
    #[doc = " Returns a boolean indicating whether or not the language was successfully"]
    #[doc = " assigned. True means assignment succeeded. False means there was a version"]
    #[doc = " mismatch: the language was generated with an incompatible version of the"]
    #[doc = " Tree-sitter CLI. Check the language\'s version using `ts_language_version`"]
    #[doc = " and compare it to this library\'s `TREE_SITTER_LANGUAGE_VERSION` and"]
    #[doc = " `TREE_SITTER_MIN_COMPATIBLE_LANGUAGE_VERSION` constants."]
    pub fn ts_parser_set_language(self_: *mut TSParser, language: *const TSLanguage) -> bool;
}
extern "C" {
    #[doc = " Get the parser\'s current language."]
    pub fn ts_parser_language(self_: *const TSParser) -> *const TSLanguage;
}
extern "C" {
    #[doc = " Set the spans of text that the parser should include when parsing."]
    #[doc = ""]
    #[doc = " By default, the parser will always include entire documents. This function"]
    #[doc = " allows you to parse only a *portion* of a document but still return a syntax"]
    #[doc = " tree whose ranges match up with the document as a whole. You can also pass"]
    #[doc = " multiple disjoint ranges."]
    #[doc = ""]
    #[doc = " The second and third parameters specify the location and length of an array"]
    #[doc = " of ranges. The parser does *not* take ownership of these ranges; it copies"]
    #[doc = " the data, so it doesn\'t matter how these ranges are allocated."]
    pub fn ts_parser_set_included_ranges(self_: *mut TSParser, ranges: *const TSRange, length: u32);
}
extern "C" {
    #[doc = " Get the ranges of text that the parser will include when parsing."]
    #[doc = ""]
    #[doc = " The returned pointer is owned by the parser. The caller should not free it"]
    #[doc = " or write to it. The length of the array will be written to the given"]
    #[doc = " `length` pointer."]
    pub fn ts_parser_included_ranges(self_: *const TSParser, length: *mut u32) -> *const TSRange;
}
extern "C" {
    #[doc = " Use the parser to parse some source code and create a syntax tree."]
    #[doc = ""]
    #[doc = " If you are parsing this document for the first time, pass `NULL` for the"]
    #[doc = " `old_tree` parameter. Otherwise, if you have already parsed an earlier"]
    #[doc = " version of this document and the document has since been edited, pass the"]
    #[doc = " previous syntax tree so that the unchanged parts of it can be reused."]
    #[doc = " This will save time and memory. For this to work correctly, you must have"]
    #[doc = " already edited the old syntax tree using the `ts_tree_edit` function in a"]
    #[doc = " way that exactly matches the source code changes."]
    #[doc = ""]
    #[doc = " The `TSInput` parameter lets you specify how to read the text. It has the"]
    #[doc = " following three fields:"]
    #[doc = " 1. `read`: A function to retrieve a chunk of text at a given byte offset"]
    #[doc = "    and (row, column) position. The function should return a pointer to the"]
    #[doc = "    text and write its length to the the `bytes_read` pointer. The parser"]
    #[doc = "    does not take ownership of this buffer; it just borrows it until it has"]
    #[doc = "    finished reading it. The function should write a zero value to the"]
    #[doc = "    `bytes_read` pointer to indicate the end of the document."]
    #[doc = " 2. `payload`: An arbitrary pointer that will be passed to each invocation"]
    #[doc = "    of the `read` function."]
    #[doc = " 3. `encoding`: An indication of how the text is encoded. Either"]
    #[doc = "    `TSInputEncodingUTF8` or `TSInputEncodingUTF16`."]
    #[doc = ""]
    #[doc = " This function returns a syntax tree on success, and `NULL` on failure. There"]
    #[doc = " are three possible reasons for failure:"]
    #[doc = " 1. The parser does not have a language assigned. Check for this using the"]
    #[doc = "`ts_parser_language` function."]
    #[doc = " 2. Parsing was cancelled due to a timeout that was set by an earlier call to"]
    #[doc = "    the `ts_parser_set_timeout_micros` function. You can resume parsing from"]
    #[doc = "    where the parser left out by calling `ts_parser_parse` again with the"]
    #[doc = "    same arguments. Or you can start parsing from scratch by first calling"]
    #[doc = "    `ts_parser_reset`."]
    #[doc = " 3. Parsing was cancelled using a cancellation flag that was set by an"]
    #[doc = "    earlier call to `ts_parser_set_cancellation_flag`. You can resume parsing"]
    #[doc = "    from where the parser left out by calling `ts_parser_parse` again with"]
    #[doc = "    the same arguments."]
    pub fn ts_parser_parse(
        self_: *mut TSParser,
        old_tree: *const TSTree,
        input: TSInput,
    ) -> *mut TSTree;
}
extern "C" {
    #[doc = " Use the parser to parse some source code stored in one contiguous buffer."]
    #[doc = " The first two parameters are the same as in the `ts_parser_parse` function"]
    #[doc = " above. The second two parameters indicate the location of the buffer and its"]
    #[doc = " length in bytes."]
    pub fn ts_parser_parse_string(
        self_: *mut TSParser,
        old_tree: *const TSTree,
        string: *const ::std::os::raw::c_char,
        length: u32,
    ) -> *mut TSTree;
}
extern "C" {
    #[doc = " Use the parser to parse some source code stored in one contiguous buffer with"]
    #[doc = " a given encoding. The first four parameters work the same as in the"]
    #[doc = " `ts_parser_parse_string` method above. The final parameter indicates whether"]
    #[doc = " the text is encoded as UTF8 or UTF16."]
    pub fn ts_parser_parse_string_encoding(
        self_: *mut TSParser,
        old_tree: *const TSTree,
        string: *const ::std::os::raw::c_char,
        length: u32,
        encoding: TSInputEncoding,
    ) -> *mut TSTree;
}
extern "C" {
    #[doc = " Instruct the parser to start the next parse from the beginning."]
    #[doc = ""]
    #[doc = " If the parser previously failed because of a timeout or a cancellation, then"]
    #[doc = " by default, it will resume where it left off on the next call to"]
    #[doc = " `ts_parser_parse` or other parsing functions. If you don\'t want to resume,"]
    #[doc = " and instead intend to use this parser to parse some other document, you must"]
    #[doc = " call this `ts_parser_reset` first."]
    pub fn ts_parser_reset(self_: *mut TSParser);
}
extern "C" {
    #[doc = " Set the maximum duration in microseconds that parsing should be allowed to"]
    #[doc = " take before halting. If parsing takes longer than this, it will halt early,"]
    #[doc = " returning NULL. See `ts_parser_parse` for more information."]
    pub fn ts_parser_set_timeout_micros(self_: *mut TSParser, timeout: u64);
}
extern "C" {
    #[doc = " Get the duration in microseconds that parsing is allowed to take."]
    pub fn ts_parser_timeout_micros(self_: *const TSParser) -> u64;
}
extern "C" {
    #[doc = " Set the parser\'s current cancellation flag pointer. If a non-null pointer is"]
    #[doc = " assigned, then the parser will periodically read from this pointer during"]
    #[doc = " parsing. If it reads a non-zero value, it will halt early, returning NULL."]
    #[doc = " See `ts_parser_parse` for more information."]
    pub fn ts_parser_set_cancellation_flag(self_: *mut TSParser, flag: *const usize);
}
extern "C" {
    #[doc = " Get the parser\'s current cancellation flag pointer."]
    pub fn ts_parser_cancellation_flag(self_: *const TSParser) -> *const usize;
}
extern "C" {
    #[doc = " Set the logger that a parser should use during parsing."]
    #[doc = ""]
    #[doc = " The parser does not take ownership over the logger payload. If a logger was"]
    #[doc = " previously assigned, the caller is responsible for releasing any memory"]
    #[doc = " owned by the previous logger."]
    pub fn ts_parser_set_logger(self_: *mut TSParser, logger: TSLogger);
}
extern "C" {
    #[doc = " Get the parser\'s current logger."]
    pub fn ts_parser_logger(self_: *const TSParser) -> TSLogger;
}
extern "C" {
    #[doc = " Set the file descriptor to which the parser should write debugging graphs"]
    #[doc = " during parsing. The graphs are formatted in the DOT language. You may want"]
    #[doc = " to pipe these graphs directly to a `dot(1)` process in order to generate"]
    #[doc = " SVG output. You can turn off this logging by passing a negative number."]
    pub fn ts_parser_print_dot_graphs(self_: *mut TSParser, file: ::std::os::raw::c_int);
}
extern "C" {
    #[doc = " Set whether or not the parser should halt immediately upon detecting an"]
    #[doc = " error. This will generally result in a syntax tree with an error at the"]
    #[doc = " root, and one or more partial syntax trees within the error. This behavior"]
    #[doc = " may not be supported long-term."]
    pub fn ts_parser_halt_on_error(self_: *mut TSParser, halt: bool);
}
extern "C" {
    #[doc = " Create a shallow copy of the syntax tree. This is very fast."]
    #[doc = ""]
    #[doc = " You need to copy a syntax tree in order to use it on more than one thread at"]
    #[doc = " a time, as syntax trees are not thread safe."]
    pub fn ts_tree_copy(self_: *const TSTree) -> *mut TSTree;
}
extern "C" {
    #[doc = " Delete the syntax tree, freeing all of the memory that it used."]
    pub fn ts_tree_delete(self_: *mut TSTree);
}
extern "C" {
    #[doc = " Get the root node of the syntax tree."]
    pub fn ts_tree_root_node(self_: *const TSTree) -> TSNode;
}
extern "C" {
    #[doc = " Get the language that was used to parse the syntax tree."]
    pub fn ts_tree_language(arg1: *const TSTree) -> *const TSLanguage;
}
extern "C" {
    #[doc = " Edit the syntax tree to keep it in sync with source code that has been"]
    #[doc = " edited."]
    #[doc = ""]
    #[doc = " You must describe the edit both in terms of byte offsets and in terms of"]
    #[doc = " (row, column) coordinates."]
    pub fn ts_tree_edit(self_: *mut TSTree, edit: *const TSInputEdit);
}
extern "C" {
    #[doc = " Compare a new syntax tree to a previous syntax tree representing the same"]
    #[doc = " document, returning an array of ranges whose syntactic structure has changed."]
    #[doc = ""]
    #[doc = " For this to work correctly, the old syntax tree must have been edited such"]
    #[doc = " that its ranges match up to the new tree. Generally, you\'ll want to call"]
    #[doc = " this function right after calling one of the `ts_parser_parse` functions,"]
    #[doc = " passing in the new tree that was returned from `ts_parser_parse` and the old"]
    #[doc = " tree that was passed as a parameter."]
    #[doc = ""]
    #[doc = " The returned array is allocated using `malloc` and the caller is responsible"]
    #[doc = " for freeing it using `free`. The length of the array will be written to the"]
    #[doc = " given `length` pointer."]
    pub fn ts_tree_get_changed_ranges(
        self_: *const TSTree,
        old_tree: *const TSTree,
        length: *mut u32,
    ) -> *mut TSRange;
}
extern "C" {
    #[doc = " Write a DOT graph describing the syntax tree to the given file."]
    pub fn ts_tree_print_dot_graph(arg1: *const TSTree, arg2: *mut FILE);
}
extern "C" {
    #[doc = " Get the node\'s type as a null-terminated string."]
    pub fn ts_node_type(arg1: TSNode) -> *const ::std::os::raw::c_char;
}
extern "C" {
    #[doc = " Get the node\'s type as a numerical id."]
    pub fn ts_node_symbol(arg1: TSNode) -> TSSymbol;
}
extern "C" {
    #[doc = " Get the node\'s start byte."]
    pub fn ts_node_start_byte(arg1: TSNode) -> u32;
}
extern "C" {
    #[doc = " Get the node\'s start position in terms of rows and columns."]
    pub fn ts_node_start_point(arg1: TSNode) -> TSPoint;
}
extern "C" {
    #[doc = " Get the node\'s end byte."]
    pub fn ts_node_end_byte(arg1: TSNode) -> u32;
}
extern "C" {
    #[doc = " Get the node\'s end position in terms of rows and columns."]
    pub fn ts_node_end_point(arg1: TSNode) -> TSPoint;
}
extern "C" {
    #[doc = " Get an S-expression representing the node as a string."]
    #[doc = ""]
    #[doc = " This string is allocated with `malloc` and the caller is responsible for"]
    #[doc = " freeing it using `free`."]
    pub fn ts_node_string(arg1: TSNode) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    #[doc = " Check if the node is null. Functions like `ts_node_child` and"]
    #[doc = " `ts_node_next_sibling` will return a null node to indicate that no such node"]
    #[doc = " was found."]
    pub fn ts_node_is_null(arg1: TSNode) -> bool;
}
extern "C" {
    #[doc = " Check if the node is *named*. Named nodes correspond to named rules in the"]
    #[doc = " grammar, whereas *anonymous* nodes correspond to string literals in the"]
    #[doc = " grammar."]
    pub fn ts_node_is_named(arg1: TSNode) -> bool;
}
extern "C" {
    #[doc = " Check if the node is *missing*. Missing nodes are inserted by the parser in"]
    #[doc = " order to recover from certain kinds of syntax errors."]
    pub fn ts_node_is_missing(arg1: TSNode) -> bool;
}
extern "C" {
    #[doc = " Check if the node is *missing*. Missing nodes are inserted by the parser in"]
    #[doc = " order to recover from certain kinds of syntax errors."]
    pub fn ts_node_is_extra(arg1: TSNode) -> bool;
}
extern "C" {
    #[doc = " Check if a syntax node has been edited."]
    pub fn ts_node_has_changes(arg1: TSNode) -> bool;
}
extern "C" {
    #[doc = " Check if the node is a syntax error or contains any syntax errors."]
    pub fn ts_node_has_error(arg1: TSNode) -> bool;
}
extern "C" {
    #[doc = " Get the node\'s immediate parent."]
    pub fn ts_node_parent(arg1: TSNode) -> TSNode;
}
extern "C" {
    #[doc = " Get the node\'s child at the given index, where zero represents the first"]
    #[doc = " child."]
    pub fn ts_node_child(arg1: TSNode, arg2: u32) -> TSNode;
}
extern "C" {
    #[doc = " Get the node\'s number of children."]
    pub fn ts_node_child_count(arg1: TSNode) -> u32;
}
extern "C" {
    #[doc = " Get the node\'s *named* child at the given index."]
    #[doc = ""]
    #[doc = " See also `ts_node_is_named`."]
    pub fn ts_node_named_child(arg1: TSNode, arg2: u32) -> TSNode;
}
extern "C" {
    #[doc = " Get the node\'s number of *named* children."]
    #[doc = ""]
    #[doc = " See also `ts_node_is_named`."]
    pub fn ts_node_named_child_count(arg1: TSNode) -> u32;
}
extern "C" {
    #[doc = " Get the node\'s child with the given field name."]
    pub fn ts_node_child_by_field_name(
        self_: TSNode,
        field_name: *const ::std::os::raw::c_char,
        field_name_length: u32,
    ) -> TSNode;
}
extern "C" {
    #[doc = " Get the node\'s child with the given numerical field id."]
    #[doc = ""]
    #[doc = " You can convert a field name to an id using the"]
    #[doc = " `ts_language_field_id_for_name` function."]
    pub fn ts_node_child_by_field_id(arg1: TSNode, arg2: TSFieldId) -> TSNode;
}
extern "C" {
    #[doc = " Get the node\'s next / previous sibling."]
    pub fn ts_node_next_sibling(arg1: TSNode) -> TSNode;
}
extern "C" {
    pub fn ts_node_prev_sibling(arg1: TSNode) -> TSNode;
}
extern "C" {
    #[doc = " Get the node\'s next / previous *named* sibling."]
    pub fn ts_node_next_named_sibling(arg1: TSNode) -> TSNode;
}
extern "C" {
    pub fn ts_node_prev_named_sibling(arg1: TSNode) -> TSNode;
}
extern "C" {
    #[doc = " Get the node\'s first child that extends beyond the given byte offset."]
    pub fn ts_node_first_child_for_byte(arg1: TSNode, arg2: u32) -> TSNode;
}
extern "C" {
    #[doc = " Get the node\'s first named child that extends beyond the given byte offset."]
    pub fn ts_node_first_named_child_for_byte(arg1: TSNode, arg2: u32) -> TSNode;
}
extern "C" {
    #[doc = " Get the smallest node within this node that spans the given range of bytes"]
    #[doc = " or (row, column) positions."]
    pub fn ts_node_descendant_for_byte_range(arg1: TSNode, arg2: u32, arg3: u32) -> TSNode;
}
extern "C" {
    pub fn ts_node_descendant_for_point_range(arg1: TSNode, arg2: TSPoint, arg3: TSPoint)
        -> TSNode;
}
extern "C" {
    #[doc = " Get the smallest named node within this node that spans the given range of"]
    #[doc = " bytes or (row, column) positions."]
    pub fn ts_node_named_descendant_for_byte_range(arg1: TSNode, arg2: u32, arg3: u32) -> TSNode;
}
extern "C" {
    pub fn ts_node_named_descendant_for_point_range(
        arg1: TSNode,
        arg2: TSPoint,
        arg3: TSPoint,
    ) -> TSNode;
}
extern "C" {
    #[doc = " Edit the node to keep it in-sync with source code that has been edited."]
    #[doc = ""]
    #[doc = " This function is only rarely needed. When you edit a syntax tree with the"]
    #[doc = " `ts_tree_edit` function, all of the nodes that you retrieve from the tree"]
    #[doc = " afterward will already reflect the edit. You only need to use `ts_node_edit`"]
    #[doc = " when you have a `TSNode` instance that you want to keep and continue to use"]
    #[doc = " after an edit."]
    pub fn ts_node_edit(arg1: *mut TSNode, arg2: *const TSInputEdit);
}
extern "C" {
    #[doc = " Check if two nodes are identical."]
    pub fn ts_node_eq(arg1: TSNode, arg2: TSNode) -> bool;
}
extern "C" {
    #[doc = " Create a new tree cursor starting from the given node."]
    #[doc = ""]
    #[doc = " A tree cursor allows you to walk a syntax tree more efficiently than is"]
    #[doc = " possible using the `TSNode` functions. It is a mutable object that is always"]
    #[doc = " on a certain syntax node, and can be moved imperatively to different nodes."]
    pub fn ts_tree_cursor_new(arg1: TSNode) -> TSTreeCursor;
}
extern "C" {
    #[doc = " Delete a tree cursor, freeing all of the memory that it used."]
    pub fn ts_tree_cursor_delete(arg1: *mut TSTreeCursor);
}
extern "C" {
    #[doc = " Re-initialize a tree cursor to start at a different ndoe."]
    pub fn ts_tree_cursor_reset(arg1: *mut TSTreeCursor, arg2: TSNode);
}
extern "C" {
    #[doc = " Get the tree cursor\'s current node."]
    pub fn ts_tree_cursor_current_node(arg1: *const TSTreeCursor) -> TSNode;
}
extern "C" {
    #[doc = " Get the field name of the tree cursor\'s current node."]
    #[doc = ""]
    #[doc = " This returns `NULL` if the current node doesn\'t have a field."]
    #[doc = " See also `ts_node_child_by_field_name`."]
    pub fn ts_tree_cursor_current_field_name(
        arg1: *const TSTreeCursor,
    ) -> *const ::std::os::raw::c_char;
}
extern "C" {
    #[doc = " Get the field name of the tree cursor\'s current node."]
    #[doc = ""]
    #[doc = " This returns zero if the current node doesn\'t have a field."]
    #[doc = " See also `ts_node_child_by_field_id`, `ts_language_field_id_for_name`."]
    pub fn ts_tree_cursor_current_field_id(arg1: *const TSTreeCursor) -> TSFieldId;
}
extern "C" {
    #[doc = " Move the cursor to the parent of its current node."]
    #[doc = ""]
    #[doc = " This returns `true` if the cursor successfully moved, and returns `false`"]
    #[doc = " if there was no parent node (the cursor was already on the root node)."]
    pub fn ts_tree_cursor_goto_parent(arg1: *mut TSTreeCursor) -> bool;
}
extern "C" {
    #[doc = " Move the cursor to the next sibling of its current node."]
    #[doc = ""]
    #[doc = " This returns `true` if the cursor successfully moved, and returns `false`"]
    #[doc = " if there was no next sibling node."]
    pub fn ts_tree_cursor_goto_next_sibling(arg1: *mut TSTreeCursor) -> bool;
}
extern "C" {
    #[doc = " Move the cursor to the first schild of its current node."]
    #[doc = ""]
    #[doc = " This returns `true` if the cursor successfully moved, and returns `false`"]
    #[doc = " if there were no children."]
    pub fn ts_tree_cursor_goto_first_child(arg1: *mut TSTreeCursor) -> bool;
}
extern "C" {
    #[doc = " Move the cursor to the first schild of its current node that extends beyond"]
    #[doc = " the given byte offset."]
    #[doc = ""]
    #[doc = " This returns the index of the child node if one was found, and returns -1"]
    #[doc = " if no such child was found."]
    pub fn ts_tree_cursor_goto_first_child_for_byte(arg1: *mut TSTreeCursor, arg2: u32) -> i64;
}
extern "C" {
    pub fn ts_tree_cursor_copy(arg1: *const TSTreeCursor) -> TSTreeCursor;
}
extern "C" {
    #[doc = " Create a new query from a string containing one or more S-expression"]
    #[doc = " patterns. The query is associated with a particular language, and can"]
    #[doc = " only be run on syntax nodes parsed with that language."]
    #[doc = ""]
    #[doc = " If all of the given patterns are valid, this returns a `TSQuery`."]
    #[doc = " If a pattern is invalid, this returns `NULL`, and provides two pieces"]
    #[doc = " of information about the problem:"]
    #[doc = " 1. The byte offset of the error is written to the `error_offset` parameter."]
    #[doc = " 2. The type of error is written to the `error_type` parameter."]
    pub fn ts_query_new(
        language: *const TSLanguage,
        source: *const ::std::os::raw::c_char,
        source_len: u32,
        error_offset: *mut u32,
        error_type: *mut TSQueryError,
    ) -> *mut TSQuery;
}
extern "C" {
    #[doc = " Delete a query, freeing all of the memory that it used."]
    pub fn ts_query_delete(arg1: *mut TSQuery);
}
extern "C" {
    #[doc = " Get the number of distinct capture names in the query."]
    pub fn ts_query_capture_count(arg1: *const TSQuery) -> u32;
}
extern "C" {
    #[doc = " Get the name and length of one of the query\'s capture. Each capture"]
    #[doc = " is associated with a numeric id based on the order that it appeared"]
    #[doc = " in the query\'s source."]
    pub fn ts_query_capture_name_for_id(
        self_: *const TSQuery,
        index: u32,
        length: *mut u32,
    ) -> *const ::std::os::raw::c_char;
}
extern "C" {
    #[doc = " Get the numeric id of the capture with the given name."]
    pub fn ts_query_capture_id_for_name(
        self_: *const TSQuery,
        name: *const ::std::os::raw::c_char,
        length: u32,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    #[doc = " Create a new context for executing a given query."]
    #[doc = ""]
    #[doc = " The context stores the state that is needed to iteratively search"]
    #[doc = " for matches. To use the query context:"]
    #[doc = " 1. First call `ts_query_context_exec` to start running the query"]
    #[doc = "    on a particular syntax node."]
    #[doc = " 2. Then repeatedly call `ts_query_context_next` to iterate over"]
    #[doc = "    the matches."]
    #[doc = " 3. After each successful call to `ts_query_context_next`, you can call"]
    #[doc = "    `ts_query_context_matched_pattern_index` to determine which pattern"]
    #[doc = "     matched. You can also call `ts_query_context_matched_captures` to"]
    #[doc = "     determine which nodes were captured by which capture names."]
    #[doc = ""]
    #[doc = " If you don\'t care about finding all of the matches, you can stop calling"]
    #[doc = " `ts_query_context_next` at any point. And you can start executing the"]
    #[doc = "  query against a different node by calling `ts_query_context_exec` again."]
    pub fn ts_query_context_new(arg1: *const TSQuery) -> *mut TSQueryContext;
}
extern "C" {
    #[doc = " Delete a query context, freeing all of the memory that it used."]
    pub fn ts_query_context_delete(arg1: *mut TSQueryContext);
}
extern "C" {
    #[doc = " Start running a query on a given node."]
    pub fn ts_query_context_exec(arg1: *mut TSQueryContext, arg2: TSNode);
}
extern "C" {
    #[doc = " Set the range of bytes or (row, column) positions in which the query"]
    #[doc = " will be executed."]
    pub fn ts_query_context_set_byte_range(arg1: *mut TSQueryContext, arg2: u32, arg3: u32);
}
extern "C" {
    pub fn ts_query_context_set_point_range(
        arg1: *mut TSQueryContext,
        arg2: TSPoint,
        arg3: TSPoint,
    );
}
extern "C" {
    #[doc = " Advance to the next match of the currently running query."]
    pub fn ts_query_context_next(arg1: *mut TSQueryContext) -> bool;
}
extern "C" {
    #[doc = " Check which pattern matched."]
    pub fn ts_query_context_matched_pattern_index(arg1: *const TSQueryContext) -> u32;
}
extern "C" {
    #[doc = " Check which pattern matched."]
    pub fn ts_query_context_matched_captures(
        arg1: *const TSQueryContext,
        arg2: *mut u32,
    ) -> *const TSQueryCapture;
}
extern "C" {
    #[doc = " Get the number of distinct node types in the language."]
    pub fn ts_language_symbol_count(arg1: *const TSLanguage) -> u32;
}
extern "C" {
    #[doc = " Get a node type string for the given numerical id."]
    pub fn ts_language_symbol_name(
        arg1: *const TSLanguage,
        arg2: TSSymbol,
    ) -> *const ::std::os::raw::c_char;
}
extern "C" {
    #[doc = " Get the numerical id for the given node type string."]
    pub fn ts_language_symbol_for_name(
        arg1: *const TSLanguage,
        arg2: *const ::std::os::raw::c_char,
    ) -> TSSymbol;
}
extern "C" {
    #[doc = " Get the number of distinct field names in the language."]
    pub fn ts_language_field_count(arg1: *const TSLanguage) -> u32;
}
extern "C" {
    #[doc = " Get the field name string for the given numerical id."]
    pub fn ts_language_field_name_for_id(
        arg1: *const TSLanguage,
        arg2: TSFieldId,
    ) -> *const ::std::os::raw::c_char;
}
extern "C" {
    #[doc = " Get the numerical id for the given field name string."]
    pub fn ts_language_field_id_for_name(
        arg1: *const TSLanguage,
        arg2: *const ::std::os::raw::c_char,
        arg3: u32,
    ) -> TSFieldId;
}
extern "C" {
    #[doc = " Check whether the given node type id belongs to named nodes, anonymous nodes,"]
    #[doc = " or a hidden nodes."]
    #[doc = ""]
    #[doc = " See also `ts_node_is_named`. Hidden nodes are never returned from the API."]
    pub fn ts_language_symbol_type(arg1: *const TSLanguage, arg2: TSSymbol) -> TSSymbolType;
}
extern "C" {
    #[doc = " Get the ABI version number for this language. This version number is used"]
    #[doc = " to ensure that languages were generated by a compatible version of"]
    #[doc = " Tree-sitter."]
    #[doc = ""]
    #[doc = " See also `ts_parser_set_language`."]
    pub fn ts_language_version(arg1: *const TSLanguage) -> u32;
}

pub const TREE_SITTER_LANGUAGE_VERSION: usize = 11;
pub const TREE_SITTER_MIN_COMPATIBLE_LANGUAGE_VERSION: usize = 9;
