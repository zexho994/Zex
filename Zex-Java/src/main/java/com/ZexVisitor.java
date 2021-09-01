// Generated from /Users/zexho/Github/Zex/Zex-Java/src/main/java/com/Zex.g4 by ANTLR 4.9.1
package com;
import org.antlr.v4.runtime.tree.ParseTreeVisitor;

/**
 * This interface defines a complete generic visitor for a parse tree produced
 * by {@link ZexParser}.
 *
 * @param <T> The return type of the visit operation. Use {@link Void} for
 * operations with no return type.
 */
public interface ZexVisitor<T> extends ParseTreeVisitor<T> {
	/**
	 * Visit a parse tree produced by {@link ZexParser#compilationUnit}.
	 * @param ctx the parse tree
	 * @return the visitor result
	 */
	T visitCompilationUnit(ZexParser.CompilationUnitContext ctx);
	/**
	 * Visit a parse tree produced by {@link ZexParser#variable}.
	 * @param ctx the parse tree
	 * @return the visitor result
	 */
	T visitVariable(ZexParser.VariableContext ctx);
	/**
	 * Visit a parse tree produced by {@link ZexParser#print}.
	 * @param ctx the parse tree
	 * @return the visitor result
	 */
	T visitPrint(ZexParser.PrintContext ctx);
	/**
	 * Visit a parse tree produced by {@link ZexParser#value}.
	 * @param ctx the parse tree
	 * @return the visitor result
	 */
	T visitValue(ZexParser.ValueContext ctx);
}