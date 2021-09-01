// Generated from /Users/zexho/Github/Zex/Zex-Java/src/main/java/com/Zex.g4 by ANTLR 4.9.1
package com;
import org.antlr.v4.runtime.tree.ParseTreeListener;

/**
 * This interface defines a complete listener for a parse tree produced by
 * {@link ZexParser}.
 */
public interface ZexListener extends ParseTreeListener {
	/**
	 * Enter a parse tree produced by {@link ZexParser#compilationUnit}.
	 * @param ctx the parse tree
	 */
	void enterCompilationUnit(ZexParser.CompilationUnitContext ctx);
	/**
	 * Exit a parse tree produced by {@link ZexParser#compilationUnit}.
	 * @param ctx the parse tree
	 */
	void exitCompilationUnit(ZexParser.CompilationUnitContext ctx);
	/**
	 * Enter a parse tree produced by {@link ZexParser#variable}.
	 * @param ctx the parse tree
	 */
	void enterVariable(ZexParser.VariableContext ctx);
	/**
	 * Exit a parse tree produced by {@link ZexParser#variable}.
	 * @param ctx the parse tree
	 */
	void exitVariable(ZexParser.VariableContext ctx);
	/**
	 * Enter a parse tree produced by {@link ZexParser#print}.
	 * @param ctx the parse tree
	 */
	void enterPrint(ZexParser.PrintContext ctx);
	/**
	 * Exit a parse tree produced by {@link ZexParser#print}.
	 * @param ctx the parse tree
	 */
	void exitPrint(ZexParser.PrintContext ctx);
	/**
	 * Enter a parse tree produced by {@link ZexParser#value}.
	 * @param ctx the parse tree
	 */
	void enterValue(ZexParser.ValueContext ctx);
	/**
	 * Exit a parse tree produced by {@link ZexParser#value}.
	 * @param ctx the parse tree
	 */
	void exitValue(ZexParser.ValueContext ctx);
}