package ast;

import token.Token;

import java.util.Collections;
import java.util.Iterator;

/**
 * @author Zexho
 * @date 2021/5/15 11:20 上午
 */
public class ASTLeaf extends ASTree {

    protected final Token token;

    public ASTLeaf(Token t) {
        this.token = t;
    }

    @Override
    public ASTree child(int i) {
        throw new IndexOutOfBoundsException();
    }

    @Override
    public int numChildren() {
        return 0;
    }

    @Override
    public Iterator<ASTree> children() {
        return Collections.emptyIterator();
    }

    @Override
    public String location() {
        return "at line " + token.getLineNumber();
    }

    public Token token() {
        return token;
    }
}
