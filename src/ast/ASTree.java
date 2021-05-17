package ast;

import java.util.Iterator;

/**
 *
 * @author Zexho
 * @date 2021/5/15 11:12 上午
 */
public abstract class ASTree implements Iterable<ASTree> {
    /**
     * get the i-th child node
     */
    public abstract ASTree child(int i);

    /**
     * get the number of children
     */
    public abstract int numChildren();

    /**
     * return an iterator for traversing child nodes
     */
    public abstract Iterator<ASTree> children();

    public abstract String location();

    @Override
    public Iterator<ASTree> iterator() {
        return children();
    }

}
