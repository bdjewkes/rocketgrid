import * as React from "react";

export interface GridProps { height: Number, width: Number, cellContents: boolean[][] }

export class Grid extends React.Component<GridProps, {}> {
    render() {
        let cells = new Array<JSX.Element>(); 
        for(var y = 0; y < this.props.height; y++){
            let row = new Array<JSX.Element>();
            for(var x = 0; x < this.props.width; x++){
                let key = x + "," + y;
                row.push((<Cell key={key} x={x} y={y} occupied={this.props.cellContents[x][y]} />));
            }
            cells.push(<tr key={y}>{row}</tr>)
        }
        return (<table><tbody>{cells}</tbody></table>);
    }
}


export interface CellProps { x: number, y: number, occupied: boolean }
export class Cell extends React.Component<CellProps, {}> {
    render(){
        var width = 10;
        var height = 10;
        var color = this.props.occupied ? "grey" : "white";
        var left = (this.props.x * width)+"px";
        var top = (this.props.y * height)+"px";
        let divStyle = {
            width:"10px",
            height:"10px",
            borderStyle:"solid",
            borderWidth:"1px",
            backgroundColor: color
            
        };
        return (<td style={divStyle}/>)
            
    }
}