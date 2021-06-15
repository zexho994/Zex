package ast;

import java.util.Iterator;
import java.util.List;

/**
 * @author Zexho
 * @date 2021/5/17 12:25 下午
 */
public class ASTList extends ASTree {
    protected List<ASTree> children;

    public ASTList(List<ASTree> list) {
        children = list;
    }

    @Override
    public ASTree child(int i) {
        return children.get(i);
    }

    @Override
    public int numChildren() {
        return children.size();
    }

    @Override
    public Iterator<ASTree> children() {
        return children.iterator();
    }

    @Override
    public String toString() {
        StringBuilder builder = new StringBuilder();
        builder.append('(');
        String sep = "";
        for (ASTree t : children) {
            builder.append(sep);
            sep = " ";
            builder.append(t.toString());
        }
        return builder.append(')').toString();
    }

    @Override
    public String location() {
        for (ASTree t : children) {
            String s = t.location();
            if (t.location() != null) {
                return s;
            }
        }
        return null;
    }

}
