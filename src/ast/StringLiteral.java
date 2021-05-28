package ast;

import token.Token;

/**
 * @author Zexho
 * @date 2021/5/28 8:36 上午
 */
public class StringLiteral extends ASTLeaf {
    public StringLiteral(Token t) {
        super(t);
    }

    public String value() {
        return token().getText();
    }
}
