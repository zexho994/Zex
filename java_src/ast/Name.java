package ast;

import token.Token;

/**
 * @author Zexho
 * @date 2021/5/17 12:33 下午
 */
public class Name extends ASTLeaf {
    public Name(Token t) {
        super(t);
    }

    public String name() {
        return token().getText();
    }
}
