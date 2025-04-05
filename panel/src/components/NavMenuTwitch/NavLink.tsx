import React, {useEffect} from "react";
import {Link, useLocation} from "react-router-dom";
import "./NavLink.css"

/**
 * @param target the url to point to, and to highlight when on
 * @param allowChildren if child paths are also considered matching, true by default
 * @param name display name/text of the link
 */
export interface NavLinkProps {
  target: string,
  allowChildren?: boolean,
  name: string
}

/**
 * Displays a Link with the given name and url in the style of our navigation menu. Is Highlighted when the page Url matches the target url, or a sub path of the url. </br>
 * E.g.: target: '/test' would match url: '/test/child'
 *
 * @param target the url to point to, and to highlight when on
 * @param allowChildren if child paths are also considered matching, true by default
 * @param name display name/text of the link
 * @constructor
 */
export default function NavLink({target, allowChildren, name}: NavLinkProps) {
  const {pathname} = useLocation();
  const [highlight, setHighlight] = React.useState(false);

  useEffect(() => {
    if (allowChildren == undefined || allowChildren) {
      setHighlight(pathname.startsWith(target));
    } else {
      setHighlight(pathname == target);
    }
  }, [allowChildren, pathname, target]);

  return <Link to={target} className={"navLink " + (highlight ? "navLinkHighlight" : "")}>{name}</Link>
}