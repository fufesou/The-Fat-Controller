//
//  TrackpadInput.swift
//  Remote
//
//  Created by Indiana Kernick on 1/2/21.
//  Copyright © 2021 Indiana Kernick. All rights reserved.
//

import UIKit;

protocol TrackpadInputDelegate: class {
    func mouseClick();
    func mouseDoubleClick();
    func mouseTripleClick();
    func mouseRightClick();
    func mouseMove(dx: Int32, dy: Int32);
    func mouseScroll(dx: Int32, dy: Int32);
    func mouseDown();
    func mouseUp();
}

class TrackpadInput: UIView, UIGestureRecognizerDelegate {
    private var initialized = false;
    private var tapOnceRecog: UITapGestureRecognizer!;
    private var tapTwiceRecog: UITapGestureRecognizer!;
    private var tapThriceRecog: UITapGestureRecognizer!;
    private var tapTwoRecog: UITapGestureRecognizer!;
    private var panOneRecog: UIPanGestureRecognizer!;
    private var panTwoRecog: UIPanGestureRecognizer!;
    private var panThreeRecog: UIPanGestureRecognizer!;
    
    private var lastPanOnePoint = CGPoint();
    private var lastPanTwoPoint = CGPoint();
    private var lastPanThreePoint = CGPoint();
    
    weak var delegate: TrackpadInputDelegate?;
    var slowMoveScale = CGFloat(1);
    var middleMoveScale = CGFloat(1);
    var fastMoveScale = CGFloat(1);
    var slowSpeed = CGFloat(1);
    var fastSpeed = CGFloat(1);
    var scrollScale = CGFloat(1);
    
    override func layoutSubviews() {
        if initialized {
            return;
        }
        initialized = true;
        
        tapOnceRecog = UITapGestureRecognizer(target: self, action: #selector(handleTapOnce(sender:)));
        tapOnceRecog.delegate = self;
        addGestureRecognizer(tapOnceRecog);
        
        tapTwiceRecog = UITapGestureRecognizer(target: self, action: #selector(handleTapTwice(sender:)));
        tapTwiceRecog.numberOfTapsRequired = 2;
        tapTwiceRecog.delegate = self;
        addGestureRecognizer(tapTwiceRecog);
        
        tapThriceRecog = UITapGestureRecognizer(target: self, action: #selector(handleTapThrice(sender:)));
        tapThriceRecog.numberOfTapsRequired = 3;
        tapThriceRecog.delegate = self;
        addGestureRecognizer(tapThriceRecog);
        
        tapTwoRecog = UITapGestureRecognizer(target: self, action: #selector(handleTapTwo(sender:)));
        tapTwoRecog.numberOfTouchesRequired = 2;
        tapTwoRecog.delegate = self;
        addGestureRecognizer(tapTwoRecog);
        
        panOneRecog = UIPanGestureRecognizer(target: self, action: #selector(handlePanOne(sender:)));
        panOneRecog.maximumNumberOfTouches = 1;
        panOneRecog.delegate = self;
        addGestureRecognizer(panOneRecog);
        
        panTwoRecog = UIPanGestureRecognizer(target: self, action: #selector(handlePanTwo(sender:)));
        panTwoRecog.minimumNumberOfTouches = 2;
        panTwoRecog.maximumNumberOfTouches = 2;
        panTwoRecog.delegate = self;
        addGestureRecognizer(panTwoRecog);
        
        panThreeRecog = UIPanGestureRecognizer(target: self, action: #selector(handlePanThree(sender:)));
        panThreeRecog.minimumNumberOfTouches = 3;
        panThreeRecog.maximumNumberOfTouches = 3;
        panThreeRecog.delegate = self;
        addGestureRecognizer(panThreeRecog);
    }
    
    @objc private func handleTapOnce(sender: UITapGestureRecognizer) {
        if sender.state == .recognized {
            delegate?.mouseClick();
        }
    }
    
    @objc private func handleTapTwice(sender: UITapGestureRecognizer) {
        if sender.state == .recognized {
            delegate?.mouseDoubleClick();
        }
    }
    
    @objc private func handleTapThrice(sender: UITapGestureRecognizer) {
        if sender.state == .recognized {
            delegate?.mouseTripleClick();
        }
    }
    
    @objc private func handleTapTwo(sender: UITapGestureRecognizer) {
        if sender.state == .recognized {
            delegate?.mouseRightClick();
        }
    }
    
    private func handlePan(lastPoint: inout CGPoint, point: CGPoint, scale: CGFloat) -> (Int32, Int32) {
        let dir = CGPoint(x: point.x - lastPoint.x, y: point.y - lastPoint.y);
        lastPoint = point;
        return (Int32(round(dir.x * scale)), Int32(round(dir.y * scale)));
    }
    
    private func getMoveScale(velocity: CGPoint) -> CGFloat {
        let speed = velocity.x * velocity.x + velocity.y * velocity.y;
        if speed < slowSpeed * slowSpeed {
            return slowMoveScale;
        } else if speed > fastSpeed * fastSpeed {
            return fastMoveScale;
        } else {
            return middleMoveScale;
        }
    }
    
    @objc private func handlePanOne(sender: UIPanGestureRecognizer) {
        if sender.state == .began {
            lastPanOnePoint = sender.translation(in: self);
        } else if sender.state == .changed {
            let scale = getMoveScale(velocity: sender.velocity(in: self));
            let (dx, dy) = handlePan(lastPoint: &lastPanOnePoint, point: sender.translation(in: self), scale: scale);
            delegate?.mouseMove(dx: dx, dy: dy);
        }
    }
    
    @objc private func handlePanTwo(sender: UIPanGestureRecognizer) {
        if sender.state == .began {
            lastPanTwoPoint = sender.translation(in: self);
        } else if sender.state == .changed {
            let (dx, dy) = handlePan(lastPoint: &lastPanTwoPoint, point: sender.translation(in: self), scale: scrollScale);
            delegate?.mouseScroll(dx: dx, dy: dy);
        }
    }
    
    @objc private func handlePanThree(sender: UIPanGestureRecognizer) {
        if sender.state == .began {
            lastPanThreePoint = sender.translation(in: self);
            delegate?.mouseDown();
        } else if sender.state == .ended {
            delegate?.mouseUp();
        } else if sender.state == .changed {
            let scale = getMoveScale(velocity: sender.velocity(in: self));
            let (dx, dy) = handlePan(lastPoint: &lastPanThreePoint, point: sender.translation(in: self), scale: scale);
            delegate?.mouseMove(dx: dx, dy: dy);
        }
    }
    
    func gestureRecognizer(_ gestureRecognizer: UIGestureRecognizer, shouldRecognizeSimultaneouslyWith otherGestureRecognizer: UIGestureRecognizer) -> Bool {
        if gestureRecognizer == tapTwoRecog {
            if otherGestureRecognizer == tapOnceRecog || otherGestureRecognizer == tapTwiceRecog {
                return false;
            }
        } else if gestureRecognizer == tapOnceRecog {
            if otherGestureRecognizer == panOneRecog || otherGestureRecognizer == tapTwoRecog || otherGestureRecognizer == tapTwiceRecog || otherGestureRecognizer == tapThriceRecog {
                return false;
            }
        } else if gestureRecognizer == tapTwiceRecog {
            if otherGestureRecognizer == tapOnceRecog {
                return false;
            }
        } else if gestureRecognizer == tapThriceRecog {
            if otherGestureRecognizer == tapOnceRecog {
                return false;
            }
        } else if gestureRecognizer == panOneRecog {
            if otherGestureRecognizer == tapOnceRecog {
                return false;
            }
        }
        return true;
    }
}